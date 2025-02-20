use crate::{
    cartridge::Header,
    error::{Error, Result},
    memory::{Access, MemoryBank},
};
use std::{fs::File, io::prelude::*};

pub fn from_file(header: &Header, file: &mut File) -> Result<MemoryBank> {
    let count = header.rom_bank_count()?;
    let size = header.rom_bank_size()?;

    file.rewind()
        .map_err(|e| Error::CartridgeLoadFailure(e.to_string()))?;

    let bank = MemoryBank::new(0..0, size, count, Access::ReadOnly);

    for i in 0..count {
        file.read_exact(bank.bank(i).borrow_mut().data_mut())
            .map_err(|e| Error::CartridgeLoadFailure(e.to_string()))?;
    }

    Ok(bank)
}
