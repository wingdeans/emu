use crate::{
    cartridge::Header,
    error::{Error, Result},
};
use std::{
    fs::File,
    io::{prelude::*, SeekFrom},
};

pub struct Rom {
    banks: Vec<Vec<u8>>,
}

impl Rom {
    pub fn from_file(header: &Header, file: &mut File) -> Result<Self> {
        let count = header.rom_bank_count()?;
        let size = header.rom_bank_size()?;

        file.seek(SeekFrom::Start(0))
            .map_err(|e| Error::CartridgeLoadFailure(e.to_string()))?;

        let mut banks = vec![vec![0u8; size as usize]; count as usize];
        for i in 0..count {
            file.read_exact(&mut banks[i as usize])
                .map_err(|e| Error::CartridgeLoadFailure(format!("{:?}", e)))?;
        }

        Ok(Self { banks })
    }

    pub fn bank(&self, bank: u32) -> Result<&[u8]> {
        if bank < self.banks.len() as u32 {
            Ok(&self.banks[bank as usize])
        } else {
            Err(Error::OutOfBoundsBank(bank))
        }
    }
}
