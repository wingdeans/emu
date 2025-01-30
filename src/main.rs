mod slaformat;
mod slaparser;
mod slareader;

use crate::slaparser::parse_sleigh;
use crate::slareader::SlaBuf;

/*
fn disassemble(decision: &Decision, byte: u8) {
    match &decision {
        Decision::Bits(start, size, options) => {
            println!("{:x} {} {}", byte, start, size);
            let idx = (byte >> 8 - (start + size)) & ((1 << size) - 1);
            disassemble(&options[idx as usize], byte)
        },
        Decision::Masks(masks) => {
            for Mask { id, off, nonzero, mask, val } in masks {
                assert_eq!(*off, 0);
                for i in 1..=*nonzero {
                    let shift = 32 - 8 * i;
                    if (mask >> shift) as u8 & byte == (val >> shift) as u8 {
                        println!("{:x} got: {}", byte, id)
                    }
                }
            }
        }
    }
}
 */

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let buf = SlaBuf::new("sm83.sla")?;
    let sleigh = parse_sleigh(buf);

    println!("{:#?}", sleigh);

    Ok(())
}
