use crate::{
    cartridge::Header,
    error::{Error, Result},
};
use std::{fs::File, io::prelude::*};

pub struct Rom {
    banks: Vec<Vec<u8>>,
}

impl Rom {
    pub fn from_file(header: &Header, file: &mut File) -> Result<Self> {
        let count = header.rom_bank_count()?;
        let size = header.rom_bank_size()?;

        file.rewind()
            .map_err(|e| Error::CartridgeLoadFailure(e.to_string()))?;

        let mut banks = Vec::<Vec<u8>>::with_capacity(count as usize);

        for _ in 0..count {
            let mut bank = vec![0u8; size];

            file.read_exact(&mut bank)
                .map_err(|e| Error::CartridgeLoadFailure(e.to_string()))?;

            banks.push(bank);
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
