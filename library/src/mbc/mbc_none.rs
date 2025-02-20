use crate::{bus::Addressable, cartridge::Header, error::Result, memory::Memory, rom::Rom};
use std::rc::Rc;

pub const ROM_BEGIN: u16 = 0x0000;
pub const ROM_END: u16 = 0x8000;
pub const RAM_BEGIN: u16 = 0xa000;
pub const RAM_END: u16 = 0xc000;

pub struct MbcNone {
    rom_bank_size: usize,
    rom: Rc<Rom>,
    ram: Memory,
}

impl MbcNone {
    pub fn new(header: &Header, rom: Rc<Rom>) -> Result<Self> {
        Ok(Self {
            rom_bank_size: header.rom_bank_size()?,
            rom,
            ram: Memory::new(RAM_BEGIN, RAM_END, header.ram_byte_size()?, true, true),
        })
    }
}

impl Addressable for MbcNone {
    fn read(&mut self, addr: u16) -> Option<u8> {
        match addr {
            ROM_BEGIN..ROM_END => {
                let bank = self
                    .rom
                    .bank(((addr as usize) / self.rom_bank_size) as u32)?;
                Some(bank[(addr as usize) % bank.len()])
            }
            _ => self.ram.read(addr),
        }
    }

    fn write(&mut self, addr: u16, value: u8) -> Option<()> {
        let lo = (addr & 0x7fff) as usize;

        match addr {
            RAM_BEGIN..RAM_END if self.ram.len() != 0 => {
                let index = (lo as usize) % self.ram.len();
                self.ram[index] = value
            }
            _ => return self.ram.write(addr, value),
        }

        Some(())
    }
}
