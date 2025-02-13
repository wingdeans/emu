use crate::{
    bus::Bus,
    cartridge::Header,
    error::{Error, Result},
    rom::Rom,
};
use std::rc::Rc;

pub const ROM_BEGIN: u16 = 0x0000;
pub const ROM_END: u16 = 0x8000;
pub const RAM_BEGIN: u16 = 0xa000;
pub const RAM_END: u16 = 0xc000;

pub struct MbcNone {
    rom: Rc<Rom>,
    ram: Vec<u8>,
}

impl MbcNone {
    pub fn new(header: &Header, rom: Rc<Rom>) -> Result<Self> {
        let ram_size = header.ram_bank_count()? * header.ram_bank_size()?;

        Ok(Self {
            rom,
            ram: vec![0u8; ram_size as usize],
        })
    }
}

impl Bus for MbcNone {
    fn read(&mut self, addr: u16) -> Result<u8> {
        match addr {
            ROM_BEGIN..ROM_END => Ok(self.rom.bank((addr >> 15) as u32)?[(addr & 0x7fff) as usize]),
            RAM_BEGIN..RAM_END => Ok(self.ram[(addr & 0x7fff) as usize]),
            _ => Err(Error::MapperReadFailure(addr)),
        }
    }

    fn write(&mut self, addr: u16, value: u8) -> Result<()> {
        match addr {
            RAM_BEGIN..RAM_END => self.ram[(addr & 0x7fff) as usize] = value,
            _ => return Err(Error::MapperWriteFailure(addr)),
        }

        Ok(())
    }
}
