mod gbloader;
mod loader;
mod sleigh;

use crate::gbloader::GBLoader;
use crate::loader::Loader;
use crate::sleigh::Sleigh;

fn main() {
    let sleigh = Sleigh::new("sm83.sla");

    let loader = GBLoader::new("../../roms/roms/porklike-gb.gb").unwrap();
    let mappings = loader.mappings().unwrap();
    println!("{:#x?}", mappings);

    sleigh.disassemble(&[0, 0, 0, 0]);
}
