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
        Ok(Self {
            rom,
            ram: vec![0u8; header.ram_byte_size()?],
        })
    }
}

impl Bus for MbcNone {
    fn read(&mut self, addr: u16) -> Result<u8> {
        let hi = addr >> 15;
        let lo = addr & 0x7fff;

        match addr {
            ROM_BEGIN..ROM_END => {
                let bank = self.rom.bank(hi as u32)?;
                Ok(bank[(lo as usize) % bank.len()])
            }
            RAM_BEGIN..RAM_END if self.ram.len() != 0 => {
                Ok(self.ram[(lo as usize) % self.ram.len()])
            }
            _ => Err(Error::MapperReadFailure(addr)),
        }
    }

    fn write(&mut self, addr: u16, value: u8) -> Result<()> {
        let lo = (addr & 0x7fff) as usize;

        match addr {
            RAM_BEGIN..RAM_END if self.ram.len() != 0 => {
                let index = (lo as usize) % self.ram.len();
                self.ram[index] = value
            }
            _ => return Err(Error::MapperWriteFailure(addr)),
        }

        Ok(())
    }
}
