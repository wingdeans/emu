use crate::{
    cartridge::Header,
    error::{Error, Result},
    memory::{Access, Memory},
};
use std::{fs::File, io::prelude::*};

pub fn from_file(header: &Header, file: &mut File) -> Result<Memory> {
    let size = header.rom_byte_size()?;

    file.rewind()
        .map_err(|e| Error::CartridgeLoadFailure(e.to_string()))?;

    let mut memory = Memory::new(size, Access::ReadOnly);
    file.read_exact(memory.data_mut())
        .map_err(|e| Error::CartridgeLoadFailure(e.to_string()))?;

    Ok(memory)
}
