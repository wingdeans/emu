mod slaformat;
mod slaparser;
mod slareader;

use crate::slaparser::{Constructor, Decision, Mask, Operand, Print, Subtable, Sym, Varlist};
use crate::slareader::SlaBuf;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

fn gen_decision(decision: Decision, constructors: &Vec<Constructor>) -> TokenStream {
    match decision {
        Decision::Bits {
            start,
            size,
            options,
        } => {
            let byte_start = (start / 8) as usize;
            let start = start % 8;
            let shift = 8 - (start + size);
            let mask = (1 << size) - 1u8;

            let range = 0..(options.len() as u8);
            let decisions = options
                .into_iter()
                .map(|decision| gen_decision(decision, constructors));

            quote! {
                match (buf[#byte_start] >> #shift) & #mask {
                    #(#range => #decisions,)*
                    _ => unreachable!()
                }
            }
        }
        Decision::Masks(masks) => {
            let branches = masks.iter().map(
                |Mask {
                     id,
                     off,
                     nonzero,
                     mask,
                     val,
                 }| {
                    assert_eq!(*off, 0);

                    let range = 0..(*nonzero as usize);
                    let (masks, vals): (Vec<u8>, Vec<u8>) = range
                        .clone()
                        .map(|i| {
                            let shift = 32 - 8 * (i + 1);
                            ((mask >> shift) as u8, (val >> shift) as u8)
                        })
                        .unzip();

                    let decode_ops = constructors[*id as usize].operands.iter().map(|op| {
                        let op_name = format_ident!("Op{}", op);
                        quote! {
                            #op_name::decode(&buf)?
                        }
                    });

                    let variant = format_ident!("Variant{}", *id);

                    quote! {
                        if #((buf[#range] & #masks == #vals))&&* {
                            Some(Self::#variant(#(#decode_ops),*))
                        }
                    }
                },
            );

            quote! {
                #(#branches)else* else {
                    None
                }
            }
        }
    }
}

fn gen_subtable(subtable: Subtable, idx: usize) -> TokenStream {
    let name = format_ident!("Sym{}", idx);
    let enum_body = subtable
        .constructors
        .iter()
        .enumerate()
        .map(|(i, constructor)| {
            let variant_name = format_ident!("Variant{}", i);
            let operands = constructor
                .operands
                .iter()
                .map(|op| format_ident!("Op{}", op));
            quote! {
                #variant_name(#(#operands),*),
            }
        });

    let decode_body = if let Some(decision) = subtable.decision {
        gen_decision(decision, &subtable.constructors)
    } else {
        let decode_ops = subtable.constructors[0].operands.iter().map(|op| {
            let op_name = format_ident!("Op{}", op);
            quote! {
                #op_name::decode(&buf)?
            }
        });
        quote! {
            Some(Self::Variant0(#(#decode_ops),*))
        }
    };

    let fmt_cases = subtable.constructors.iter().map(|constructor| {
        let variant = format_ident!("Variant{}", constructor.id);
        let operand_bindings = (0..constructor.operands.len()).map(|i| format_ident!("op{}", i));

        let operand_args = constructor.prints.iter().filter_map(|print| match print {
            Print::Print(_) => None,
            Print::OpPrint(idx) => Some(format_ident!("op{}", idx)),
        });

        let fstring = constructor
            .prints
            .iter()
            .map(|print| match print {
                Print::Print(piece) => piece,
                Print::OpPrint(_) => "{}",
            })
            .collect::<Vec<_>>()
            .join("");

        /* Print::OpPrint(op_idx) => {
            let sym_idx = constructor.operands[*op_idx as usize];
            let Sym::Op(op) = sleigh.get_sym(sym_idx) else {
            unreachable!()
        };
            match op {
            t @ Operand::Tok(_) => "Token".to_string(), //format!("{:?}:{:?}", sym_idx, t),
            Operand::Subsym {
            subsym: subsym_idx, ..
        } => match sleigh.get_sym(*subsym_idx) {
            Sym::Subtable { .. } => "Subtable".to_string(),
            Sym::Varnode => sleigh.get_sym_name(*subsym_idx).to_string(),
            Sym::Varlist { .. } => "VARLIST".to_string(),
            other => format!("{:?}=>{:?}:{:?}", sym_idx, subsym_idx, other),
        },
            Operand::Unk => "<<UNKNOWN>>".to_string(),
        }
        } */

        quote! {
            Self::#variant(#(#operand_bindings),*) =>
                write!(f, #fstring, #(#operand_args),*),
        }
    });

    quote! {
        #[derive(Debug)]
        enum #name {
            #(#enum_body)*
        }

        impl #name {
            #[allow(unused_parens)] // TODO
            #[allow(unused_variables)]
            fn decode(buf: &[u8]) -> Option<Self> {
                #decode_body
            }
        }

        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match self {
                    #(#fmt_cases)*
                }
            }
        }
    }
}

fn gen_operand(op: Operand, idx: usize) -> TokenStream {
    let name = format_ident!("Op{}", idx);

    let struct_body = match op {
        Operand::Subsym { subsym, .. } => Some(format_ident!("Sym{}", subsym)),
        _ => None,
    };

    let decode_arg = match op {
        Operand::Subsym { subsym, .. } => {
            let subsym = format_ident!("Sym{}", subsym);
            Some(quote! { #subsym::decode(buf)? })
        }
        _ => None,
    };

    let write = match op {
        Operand::Subsym { .. } => quote!("{}", self.0),
        Operand::Tok(_) => quote!("tok"),
        _ => quote!("UNK?"),
    };

    quote! {
        #[derive(Debug)]
        struct #name(#struct_body);

        impl #name {
            #[allow(unused_variables)]
            fn decode(buf: &[u8]) -> Option<Self> {
                Some(Self(#decode_arg))
            }
        }

        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, #write)
            }
        }
    }
}

fn gen_varnode(text: &str, idx: usize) -> TokenStream {
    let name = format_ident!("Sym{}", idx);
    quote! {
        #[derive(Debug)]
        struct #name();

        impl #name {
            fn decode(_: &[u8]) -> Option<Self> {
                Some(Self())
            }
        }

        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, #text)
            }
        }
    }
}

fn gen_varlist(varlist: Varlist, idx: usize) -> TokenStream {
    let name = format_ident!("Sym{}", idx);
    quote! {
        #[derive(Debug)]
        struct #name();

        impl #name {
            fn decode(_: &[u8]) -> Option<Self> {
                Some(Self())
            }
        }

        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "varlist")
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let buf = SlaBuf::new("sm83.sla")?;
    let sleigh = buf.parse_sleigh();

    let mut tokens = quote! {
        #[derive(Debug)]
        pub(crate) struct Insn(Sym0);

        pub(crate) fn decode(buf: &[u8]) -> Option<Insn> {
            Sym0::decode(buf).map(|st| Insn(st))
        }

        impl std::fmt::Display for Insn {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }
    };

    for (i, sym) in sleigh.syms.into_iter().enumerate() {
        match sym {
            Sym::Subtable(subtable) => tokens.extend(gen_subtable(subtable, i)),
            Sym::Op(operand) => tokens.extend(gen_operand(operand, i)),
            Sym::Varnode => tokens.extend(gen_varnode(&sleigh.sym_names[i], i)),
            Sym::Varlist(varlist) => tokens.extend(gen_varlist(varlist, i)),
            _ => (),
        }
    }

    println!("{}", prettyplease::unparse(&syn::parse2(tokens)?));

    Ok(())
}
