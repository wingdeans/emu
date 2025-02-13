use crate::{
    bus::Bus,
    cartridge::Cartridge,
    error::{Error, Result},
};

mod locations {
    pub const ROM_BANK_X0_BEGIN: u16 = 0x0000;
    pub const ROM_BANK_X0_END: u16 = 0x4000;
    pub const ROM_BANK_01_7F_BEGIN: u16 = 0x4000;
    pub const ROM_BANK_01_7F_END: u16 = 0x8000;
    pub const RAM_BANK_00_03_BEGIN: u16 = 0xa000;
    pub const RAM_BANK_00_03_END: u16 = 0xc000;
    pub const RAM_ENABLE_BEGIN: u16 = 0x0000;
    pub const RAM_ENABLE_END: u16 = 0x2000;
    pub const ROM_BANK_SELECT_BEGIN: u16 = 0x2000;
    pub const ROM_BANK_SELECT_END: u16 = 0x4000;
    pub const RAM_BANK_SELECT_BEGIN: u16 = 0x4000;
    pub const RAM_BANK_SELECT_END: u16 = 0x6000;
    pub const MODE_SELECT_BEGIN: u16 = 0x6000;
    pub const MODE_SELECT_END: u16 = 0x8000;
}

pub struct Mbc1 {
    cartridge: Cartridge,
    rom_bank: u32,
    ram_enable: bool,
    ram_bank: u32,
    ram_banks: Vec<Vec<u8>>,
    advanced_banking: bool,
}

impl Mbc1 {
    pub fn new(cartridge: Cartridge) -> Result<Self> {
        let ram_bank_count = cartridge.header().ram_bank_count()?;
        let ram_bank_size = cartridge.header().ram_bank_size()?;

        Ok(Self {
            cartridge,
            rom_bank: 1,
            ram_enable: false,
            ram_bank: 0,
            ram_banks: vec![vec![0; ram_bank_size as usize]; ram_bank_count as usize],
            advanced_banking: false,
        })
    }
}

impl Bus for Mbc1 {
    fn read(&mut self, addr: u16) -> Result<u8> {
        use locations::*;

        match addr {
            ROM_BANK_X0_BEGIN..ROM_BANK_X0_END => {
                let bank = if self.advanced_banking {
                    self.rom_bank
                } else {
                    0
                };

                Ok(self.cartridge.rom_bank(bank)?[(addr & 0x1fff) as usize])
            }
            ROM_BANK_01_7F_BEGIN..ROM_BANK_01_7F_END => {
                Ok(self.cartridge.rom_bank(self.rom_bank)?[(addr & 0x1fff) as usize])
            }
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
            ROM_BANK_SELECT_BEGIN..ROM_BANK_SELECT_END => {
                let mask = value & 0b11111;
                self.rom_bank = if mask == 0 {
                    1
                } else {
                    (mask as u32).clamp(1, self.cartridge.header().rom_bank_count()?)
                };

                Ok(())
            }
            RAM_BANK_SELECT_BEGIN..RAM_BANK_SELECT_END => {
                self.ram_bank = (value & 0b11) as u32;
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
