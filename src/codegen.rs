use crate::slamodel::{
    Constructor, Decision, Expr, Mask, OpExpr, Operand, Print, Sleigh, Subtable, Sym, SymbolTable,
    TokenField, Varlist,
};

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

fn gen_decision(decision: &Decision, constructors: &Vec<Constructor>) -> TokenStream {
    match decision {
        Decision::Bits {
            start,
            size,
            options,
        } => {
            // TODO: multi-byte
            let byte_start = (start / 8) as usize;
            let start = start % 8;
            let shift = 8 - (start + size);
            let mask = (1 << size) - 1u8;

            let range = 0..(options.len() as u8);
            let decisions = options
                .iter()
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
                            #op_name::decode(buf)?
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

fn gen_tokenfield(tokenfield: &TokenField, off: u8) -> TokenStream {
    let offset = off as usize;
    let TokenField {
        startbit,
        endbit,
        startbyte,
        endbyte,
        shift,
    } = tokenfield;
    let endbit = std::cmp::min(endbit + 1, 8); // TODO
    let start = (1u32 << startbit) - 1;
    let end = (1u32 << endbit) - 1;
    let mask: u8 = ((end - start) >> shift).try_into().unwrap();
    // println!("{} {} {} {:08b}", startbit, endbit, shift, mask);
    quote! {
        (buf[#offset] >> #shift & #mask) // TODO
    }
}

// TOPLEVEL

fn gen_subtable(subtable: &Subtable, symtab: &SymbolTable, idx: usize) -> TokenStream {
    let name = format_ident!("Sym{}", idx);
    let enum_variants = subtable
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

    let decode_body = if let Some(decision) = &subtable.decision {
        gen_decision(decision, &subtable.constructors)
    } else {
        let decode_ops = subtable.constructors[0].operands.iter().map(|op| {
            let op_name = format_ident!("Op{}", op);
            quote! {
                #op_name::decode(buf)?
            }
        });
        quote! {
            Some(Self::Variant0(#(#decode_ops),*))
        }
    };

    let fmt_cases = subtable
        .constructors
        .iter()
        .enumerate()
        .map(|(id, constructor)| {
            let variant = format_ident!("Variant{}", id);
            let operand_bindings =
                (0..constructor.operands.len()).map(|i| format_ident!("op{}", i));

            // TODO: coalesce prints
            let writes = constructor.prints.iter().map(|print| {
                match print {
                    Print::Print(piece) => quote! {
                        write!(f, #piece)?;
                    },
                    Print::OpPrint(idx) => {
                        let op_ident = format_ident!("op{}", idx);
                        let op_idx = constructor.operands[*idx as usize];
                        if let Sym::Op(op) = symtab.get_sym(op_idx) {
                            let args = op.args.iter().map(|(_, _, idx)| {
                                let arg_ident = format_ident!("op{}", idx);
                                quote!(#arg_ident.0 as u64) // TODO
                            });
                            quote! {
                                #op_ident.write(f, #(#args),*)?;
                            }
                        } else {
                            quote! {
                                #op_ident.write(f)?;
                            }
                        }
                    }
                }
            });

            quote! {
                Self::#variant(#(#operand_bindings),*) => {
                    #(#writes)*
                    Ok(())
                }
            }
        });

    let pcode_cases = subtable
        .constructors
        .iter()
        .enumerate()
        .map(|(id, constructor)| {
            let variant = format_ident!("Variant{}", id);
            let operand_bindings =
                (0..constructor.operands.len()).map(|i| format_ident!("op{}", i));

            let op = constructor
                .construct_tpl
                .iter()
                .map(|op_tpl| format!("{:?}", op_tpl))
                .collect::<Vec<_>>()
                .join(", ");

            quote! {
                Self::#variant(#(#operand_bindings),*) => #op,
            }
        });

    quote! {
        enum #name {
            #(#enum_variants)*
        }

        impl #name {
            fn decode(buf: &[u8]) -> Option<Self> {
                #decode_body
            }

            fn pcode(&self, vec: &mut Vec<Pcode>) {
                println!("    {}", match self {
                    #(#pcode_cases)*
                })
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

fn gen_expr(expr: &Expr) -> TokenStream {
    match expr {
        Expr::Lshift(a, b) => {
            let a = gen_expr(a);
            let b = gen_expr(b);
            quote! {
                (#a << #b)
            }
        }
        Expr::Plus(a, b) => {
            let a = gen_expr(a);
            let b = gen_expr(b);
            quote! {
                (#a + #b)
            }
        }
        Expr::Minus(a) => {
            let a = gen_expr(a);
            quote! {
                (-(#a as i64))
            }
        }
        Expr::Const(a) => quote!(#a),
        Expr::InsnEnd => quote!(0), // TODO
        Expr::Operand(i) => {
            let ident = format_ident!("arg{}", i);
            quote!(#ident)
        }
    }
}

fn gen_operand(op: &Operand, idx: usize) -> TokenStream {
    let name = format_ident!("Op{}", idx);

    let struct_body = match &op.expr {
        OpExpr::Subsym(subsym) => Some(format_ident!("Sym{}", subsym)),
        OpExpr::Tok(_) => Some(format_ident!("u8")),
        _ => None,
    };

    let write_args = (0..op.args.len()).map(|i| {
        let name = format_ident!("arg{}", i);
        quote! {
            #name: u64
        }
    });

    let write = match &op.expr {
        OpExpr::Subsym(_) => quote!("{}", self.0),
        OpExpr::Tok(_) => quote!("0x{:X}", self.0),
        OpExpr::Expr(expr) => {
            let expr = gen_expr(expr);
            quote!("{}", #expr)
        }
    };

    let decode_arg = match &op.expr {
        OpExpr::Subsym(subsym) => {
            let offset = op.off as usize;
            let subsym = format_ident!("Sym{}", subsym);
            Some(quote! { #subsym::decode(&buf[#offset..])? })
        }
        OpExpr::Tok(tokenfield) => Some(gen_tokenfield(tokenfield, op.off)),
        _ => None,
    };

    quote! {
        struct #name(#struct_body);

        impl #name {
            fn decode(buf: &[u8]) -> Option<Self> {
                Some(Self(#decode_arg))
            }

            fn write(&self, f: &mut std::fmt::Formatter, #(#write_args),*) -> std::fmt::Result {
                write!(f, #write)
            }
        }
    }
}

fn gen_varnode(text: &str, idx: usize) -> TokenStream {
    let name = format_ident!("Sym{}", idx);

    quote! {
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

fn gen_varlist(varlist: &Varlist, idx: usize) -> TokenStream {
    let name = format_ident!("Sym{}", idx);
    let enum_body = varlist
        .vars
        .iter()
        .enumerate()
        .filter_map(|(i, maybe_idx)| {
            let variant_name = format_ident!("Variant{}", i);
            maybe_idx.map(|idx| {
                let variant_body = format_ident!("Sym{}", idx);
                Some(quote! {
                    #variant_name(#variant_body),
                })
            })
        });

    let decode_arms = varlist
        .vars
        .iter()
        .enumerate()
        .filter_map(|(i, maybe_idx)| {
            maybe_idx.map(|idx| {
                let variant_name = format_ident!("Variant{}", i);
                let sym_name = format_ident!("Sym{}", idx);
                let pat = i as u8;
                Some(quote! {
                    #pat => Some(Self::#variant_name(#sym_name::decode(buf)?)),
                })
            })
        });

    let write_arms = varlist
        .vars
        .iter()
        .enumerate()
        .filter_map(|(i, maybe_idx)| {
            maybe_idx.map(|_| {
                let variant_name = format_ident!("Variant{}", i);
                Some(quote! {
                    Self::#variant_name(x) => x.fmt(f),
                })
            })
        });

    let tokenfield = gen_tokenfield(&varlist.tokenfield, 0);

    quote! {
        enum #name {
            #(#enum_body)*
        }

        impl #name {
            fn decode(buf: &[u8]) -> Option<Self> {
                match #tokenfield {
                    #(#decode_arms)*
                    _ => None
                }
            }
        }

        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match self {
                    #(#write_arms)*
                }
            }
        }
    }
}

pub(crate) fn emit(sleigh: Sleigh) -> Result<(), Box<dyn std::error::Error>> {
    let tokens = quote! {
        #![allow(unused_variables)]
        #![allow(dead_code)]
        #![allow(unused_parens)]
        #![allow(clippy::identity_op)]
        #![allow(clippy::double_parens)]
        #![allow(clippy::ptr_arg)] // TODO
        pub(crate) struct Pcode();
        pub(crate) struct Insn(Sym0);

        pub(crate) fn decode(buf: &[u8]) -> Option<Insn> {
            Sym0::decode(buf).map(Insn)
        }

        impl Insn {
            pub(crate) fn pcode(&self, vec: &mut Vec<Pcode>) {
                self.0.pcode(vec)
            }
        }

        impl std::fmt::Display for Insn {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }
    };
    println!("{}", prettyplease::unparse(&syn::parse2(tokens)?));

    for (i, sym) in sleigh.symtab.syms.iter().enumerate() {
        let tokens = match sym {
            Sym::Subtable(subtable) => gen_subtable(subtable, &sleigh.symtab, i),
            Sym::Op(operand) => gen_operand(operand, i),
            Sym::Varnode => gen_varnode(&sleigh.symtab.sym_names[i], i),
            Sym::Varlist(varlist) => gen_varlist(varlist, i),
            _ => continue,
        };
        println!("{}", prettyplease::unparse(&syn::parse2(tokens)?));
    }

    Ok(())
}
