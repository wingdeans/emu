mod gbloader;
mod pcode;
mod sleigh;

use crate::gbloader::GBLoader;
use crate::pcode::Pcode;
use crate::sleigh::Sleigh;

fn main() -> anyhow::Result<()> {
    let sleigh = Sleigh::new("sm83.sla");

    let loader = GBLoader::new("../../roms/roms/porklike-gb.gb")?;
    let _mappings = loader.mappings()?;

    let mut pcode_buf = Vec::new();
    let mut pc = 0x100;

    loop {
        let (a, b) = sleigh.disassemble(&loader.0[pc as usize..], pc);
        println!("0x{:x} {} {}", pc, a, b);
        let pcode_start = pcode_buf.len();
        pc += sleigh.pcode(&loader.0[pc as usize..], pc, &mut pcode_buf) as u64;

        for pcode in &pcode_buf[pcode_start..] {
            match pcode {
                Pcode::Branch(target) => pc = target.offset,
                Pcode::Call(target) => pc = target.offset,
                _ => (),
            }
        }
        println!("\t{:x?}", &pcode_buf[pcode_start..]);

        if pc == 0x38 {
            break;
        }
    }

    Ok(())
}
