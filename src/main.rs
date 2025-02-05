mod slaformat;
mod slaparser;
mod slareader;

use crate::slaparser::{Decision, Mask, Print, Sym};
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
    let sleigh = buf.parse_sleigh();

    let Sym::Subtable {
        ref decision,
        ref constructors,
    } = sleigh.syms[sleigh.subtables["instruction"]]
    else {
        unreachable!();
    };

    let decision_body = gen_decision(decision);

    let print_cases = constructors.iter().map(|constructor| {
        let Print::Print(s) = &constructor.prints[0] else {
            unreachable!();
        };
        let id = constructor.id;

        for operand in &constructor.operands {
            println!("{:?}", sleigh.syms[*operand as usize]);
        }

        quote! {
            #id => #s,
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
    // println!("{}", prettyplease::unparse(&syn::parse2(tokens)?));

    Ok(())
}
