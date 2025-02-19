mod gbloader;
mod sleigh;

use crate::gbloader::{GBLoader, Mapping};
use crate::sleigh::Sleigh;

fn main() -> anyhow::Result<()> {
    let sleigh = Sleigh::new("sm83.sla");

    let loader = GBLoader::new("../../roms/roms/porklike-gb.gb")?;
    let mappings = loader.mappings()?;

    let (a, b) = sleigh.disassemble(&loader.0[0x100..], 0x100);
    println!("{} {}", a, b);

    sleigh.pcode(&loader.0[0x100..], 0x100);
    let mut i = 0x153;
    for j in 0..10 {
        println!("{}: {:x}", j, i);
        let (a, b) = sleigh.disassemble(&loader.0[i..], i);
        println!("{} {}", a, b);
        i += sleigh.pcode(&loader.0[i..], i) as usize;
    }

    Ok(())
}
