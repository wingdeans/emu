use error::{Error, Result};

pub enum Mapper {
    MBC1,
    MBC5,
}

#[repr(packed)]
pub struct Header {
    p0: [u8; 0x100],                // 0x000..=0x099
    pub entrypoint: [u8; 4],        // 0x100,.=0x103
    pub nintendo_logo: [u8; 48],    // 0x104,.=0x133
    pub title: [u8; 10],            // 0x134,.=0x13e
    pub manufacturer_code: [u8; 4], // 0x13f..=0x142
    pub cgb_flag: u8,               // 0x143
    pub new_licensee_code: [u8; 2], // 0x144..=0x145
    pub sgb_flag: u8,               // 0x146
    pub cartridge_type: u8,         // 0x147
    pub rom_size: u8,               // 0x148
}

impl Header {
    pub fn rom_byte_size(&self) -> Result<usize> {
        match self.rom_size {
            0..=8 => Ok(32 * 1024 * (1 << self.rom_size)),
            _ => Err(Error::UnsupportedRomSize(self.rom_size)),
        }
    }

    pub fn rom_banks(&self) -> Result<u32> {
        match self.rom_size {
            0..=8 => Ok(2_u32.pow(self.rom_size as u32)),
            _ => Err(Error::UnsupportedRomSize(self.rom_size)),
        }
    }
}
