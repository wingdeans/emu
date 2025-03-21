use crate::{
    bus::Addressable,
    error::{Error, Result},
    mbc::{mbc5::Mbc5, mbc_none::MbcNone},
    rom,
};
use std::{cell::RefCell, fs::File, io::prelude::*, path::Path, ptr, rc::Rc};

pub const ENTRYPOINT_ADDRESS: usize = 0x100;
pub const HEADER_BEGIN: usize = 0x000;
pub const HEADER_END: usize = 0x150;
pub const HEADER_SIZE: usize = HEADER_END - HEADER_BEGIN;
pub const CHECKSUM_BEGIN: usize = 0x134;
pub const CHECKSUM_END: usize = 0x14d;

pub const CGB_FLAG_ADDRESS: u16 = 0x0143;

pub enum Mapper {
    NoMbc,
    Mbc5,
}

#[repr(packed)]
#[derive(Copy, Clone)]
pub struct Header {
    _p0: [u8; 0x100],                // 0x000..=0x099
    pub entrypoint: [u8; 4],         // 0x100..=0x103
    pub nintendo_logo: [u8; 48],     // 0x104..=0x133
    pub title: [u8; 11],             // 0x134..=0x13e
    pub manufacturer_code: [u8; 4],  // 0x13f..=0x142
    pub cgb_flag: u8,                // 0x143
    pub new_licensee_code: [u8; 2],  // 0x144..=0x145
    pub sgb_flag: u8,                // 0x146
    pub cartridge_type: u8,          // 0x147
    pub rom_size: u8,                // 0x148
    pub ram_size: u8,                // 0x149
    pub destination_code: u8,        // 0x14a
    pub old_licensee_code: u8,       // 0x14b
    pub mask_rom_version_number: u8, // 0x14c
    pub header_checksum: u8,         // 0x14d
    pub global_checksum: [u8; 2],    // 0x14e..=0x14f
}

pub struct Cartridge {
    mapper: Rc<RefCell<dyn Addressable>>,
}

impl Header {
    pub fn from(bytes: &[u8]) -> Result<Self> {
        let header: Header = unsafe { ptr::read_unaligned(bytes.as_ptr() as *const _) };

        let mut checksum: u8 = 0;
        for i in CHECKSUM_BEGIN..CHECKSUM_END {
            checksum = checksum.wrapping_sub(bytes[i]).wrapping_sub(1);
        }

        if checksum != header.header_checksum {
            Err(Error::InvalidCartridgeHeaderChecksum {
                header: header.header_checksum,
                calculated: checksum,
            })
        } else {
            Ok(header)
        }
    }

    pub fn rom_byte_size(&self) -> Result<usize> {
        match self.rom_size {
            0..=8 => Ok(32 * 1024 * (1 << self.rom_size)),
            _ => Err(Error::UnrecognizedCartridgeHeaderField(format!(
                "unrecognized rom size: 0x{:02x}",
                self.rom_size
            ))),
        }
    }

    pub fn rom_bank_size(&self) -> Result<usize> {
        Ok(self.rom_byte_size()? / (self.rom_bank_count()? as usize))
    }

    pub fn rom_bank_count(&self) -> Result<u32> {
        match self.rom_size {
            0..=4 => Ok(2_u32.pow(self.rom_size as u32 + 1)),
            _ => Err(Error::UnrecognizedCartridgeHeaderField(format!(
                "unrecognized rom size: 0x{:02x}",
                self.rom_size
            ))),
        }
    }

    pub fn ram_byte_size(&self) -> Result<usize> {
        Ok(self.ram_bank_size()? * (self.ram_bank_count()? as usize))
    }

    pub fn ram_bank_size(&self) -> Result<usize> {
        Ok(1024 * 8)
    }

    pub fn ram_bank_count(&self) -> Result<u32> {
        match self.ram_size {
            0 => Ok(0),
            2 => Ok(1),
            3 => Ok(4),
            4 => Ok(16),
            5 => Ok(8),
            _ => Err(Error::UnrecognizedCartridgeHeaderField(format!(
                "unrecognized ram size: 0x{:02x}",
                self.ram_size
            ))),
        }
    }

    pub fn mapper(&self) -> Result<Mapper> {
        use Mapper::*;

        match self.cartridge_type {
            0 => Ok(NoMbc),
            0x1a | 0x1b | 0x1c | 0x1d | 0x1e => Ok(Mbc5),
            _ => Err(Error::UnrecognizedCartridgeHeaderField(format!(
                "unrecognized cartridge type: 0x{:02x}",
                self.cartridge_type
            ))),
        }
    }
}

impl Cartridge {
    pub fn load_from_file(path: &Path) -> Result<Self> {
        let mut file = File::open(path).map_err(|e| Error::CartridgeLoadFailure(e.to_string()))?;

        let mut buffer = [0u8; HEADER_SIZE];
        file.read_exact(&mut buffer)
            .map_err(|e| Error::CartridgeLoadFailure(e.to_string()))?;
        let header = Header::from(&buffer)?;

        let rom = Rc::new(RefCell::new(rom::from_file(&header, &mut file)?));

        let mapper = match header.mapper()? {
            Mapper::NoMbc => {
                Rc::new(RefCell::new(MbcNone::new(&header, rom)?)) as Rc<RefCell<dyn Addressable>>
            }
            Mapper::Mbc5 => {
                Rc::new(RefCell::new(Mbc5::new(&header, rom)?)) as Rc<RefCell<dyn Addressable>>
            }
        };

        Ok(Self { mapper })
    }
}

impl Addressable for Cartridge {
    fn read(&mut self, addr: u16) -> Option<u8> {
        self.mapper.borrow_mut().read(addr)
    }

    fn write(&mut self, addr: u16, value: u8) -> Option<()> {
        self.mapper.borrow_mut().write(addr, value)
    }
}
