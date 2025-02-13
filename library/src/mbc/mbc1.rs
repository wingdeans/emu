use crate::{
    bus::Bus,
    cartridge::{self, Cartridge},
    error::{Error, Result},
};

mod locations {
    pub const RAM_BANK_00_03_BEGIN: u16 = 0xa000;
    pub const RAM_BANK_00_03_END: u16 = 0xc000;
    pub const RAM_ENABLE_BEGIN: u16 = 0x0000;
    pub const RAM_ENABLE_END: u16 = 0x2000;
    pub const ROM_BANK_BEGIN: u16 = 0x2000;
    pub const ROM_BANK_END: u16 = 0x4000;
    pub const RAM_BANK_BEGIN: u16 = 0x4000;
    pub const RAM_BANK_END: u16 = 0x6000;
    pub const MODE_SELECT_BEGIN: u16 = 0x6000;
    pub const MODE_SELECT_END: u16 = 0x8000;
}

pub struct Mbc1 {
    cartridge: Cartridge,
    ram_enable: bool,
    ram_bank: u8,
    ram_banks: Vec<Vec<u8>>,
    advanced_banking: bool,
}

impl Bus for Mbc1 {
    fn read(&mut self, addr: u16) -> Result<u8> {
        use locations::*;

        match addr {
            RAM_BANK_00_03_BEGIN..RAM_BANK_00_03_END => {
                if self.ram_enable {
                    Ok(self.ram_banks[self.ram_bank as usize][(addr & 0x0fff) as usize])
                } else {
                    Ok(0xff)
                }
            }
            _ => Err(Error::MapperReadFailure(addr)),
        }
    }

    fn write(&mut self, addr: u16, value: u8) -> Result<()> {
        use locations::*;

        match addr {
            RAM_ENABLE_BEGIN..RAM_ENABLE_END => {
                self.ram_enable = (value & 0x0f) == 0xa;
                Ok(())
            }
            ROM_BANK_BEGIN..ROM_BANK_END => {}
            RAM_BANK_BEGIN..RAM_BANK_END => {
                self.ram_bank = value & 0b11;
                Ok(())
            }
            MODE_SELECT_BEGIN..MODE_SELECT_END => {
                self.advanced_banking = (value & 1) != 0;
                Ok(())
            }
            _ => Err(Error::MapperWriteFailure(addr)),
        }
    }
}
