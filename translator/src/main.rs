mod gbloader;
mod sleigh;

use crate::gbloader::GBLoader;
use crate::sleigh::Sleigh;

fn main() -> anyhow::Result<()> {
    let sleigh = Sleigh::new("sm83.sla");

    let loader = GBLoader::new("../../roms/roms/porklike-gb.gb")?;
    let mappings = loader.mappings()?;

    let (a, b) = sleigh.disassemble(&[0, 0, 0, 0], 0);
    println!("{} {}", a, b);

    Ok(())
}
