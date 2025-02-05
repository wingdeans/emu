mod slaformat;
mod slaparser;
mod slareader;

use crate::slaparser::{Decision, Mask, Operand, Print, Sym};
use crate::slareader::SlaBuf;

use proc_macro2::TokenStream;
use quote::quote;

fn gen_decision(decision: &Decision) -> TokenStream {
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

            let decisions = options.iter().map(gen_decision);
            let range = 0..(options.len() as u8);

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

                    quote! {
                        if #((buf[#range] & #masks == #vals))&&* {
                            Some(#id)
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let buf = SlaBuf::new("sm83.sla")?;
    let mut sleigh = buf.parse_sleigh();

    let Sym::Subtable {
        ref decision,
        ref constructors,
    } = sleigh.find_sym("instruction").unwrap()
    else {
        unreachable!();
    };

    let decision_body = gen_decision(&decision);

    let print_cases = constructors.into_iter().map(|constructor| {
        let id = constructor.id;
        let s = constructor
            .prints
            .iter()
            .map(|p| match p {
                Print::Print(s) => s.clone(), // TODO
                Print::OpPrint(op_idx) => {
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
                }
            })
            .collect::<Vec<_>>()
            .join("");

        quote! {
            #id => #s,
            0 => #s,
        }
    });

    let tokens = quote! {
        #[allow(unused_parens)]
        pub(crate) fn decode(buf: &[u8]) -> Option<u16> {
            #decision_body
        }

        pub(crate) fn print(op: u16) -> &'static str {
            match op {
                #(#print_cases)*
                _ => unreachable!()
            }
        }
    };

    // println!("{:#?}", sleigh.operands);
    println!("{}", prettyplease::unparse(&syn::parse2(tokens)?));

    Ok(())
}
