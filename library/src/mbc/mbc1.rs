use crate::{
    bus::{bank, map_from, map_to, Addressable, Bank, Bus},
    cartridge::Header,
    error::Result,
    memory::{Access, Memory},
};
use std::{cell::RefCell, rc::Rc};

pub const ROM_BANKX0_BEGIN: u16 = 0x0000;
pub const ROM_BANKX0_END: u16 = 0x4000;
pub const ROM_BANKX0_SIZE: usize = (ROM_BANKX0_END - ROM_BANKX0_BEGIN) as usize;
pub const ROM_BANKXX_BEGIN: u16 = 0x4000;
pub const ROM_BANKXX_END: u16 = 0x8000;
pub const ROM_BANKXX_SIZE: usize = (ROM_BANKXX_END - ROM_BANKXX_BEGIN) as usize;
pub const RAM_BEGIN: u16 = 0xa000;
pub const RAM_END: u16 = 0xc000;
pub const RAM_SIZE: usize = (RAM_END - RAM_BEGIN) as usize;
pub const RAM_ENABLE_BEGIN: u16 = 0x0000;
pub const RAM_ENABLE_END: u16 = 0x2000;
pub const RAM_BANK_BEGIN: u16 = 0x4000;
pub const RAM_BANK_END: u16 = 0x6000;
pub const ROM_BANK_BEGIN: u16 = 0x2000;
pub const ROM_BANK_END: u16 = 0x4000;
pub const BANK_MODE_BEGIN: u16 = 0x6000;
pub const BANK_MODE_END: u16 = 0x8000;

pub struct Mbc1 {
    bus: Bus,
    ram_bank: Option<Rc<RefCell<Bank>>>,
    ram_map: Option<Rc<RefCell<dyn Addressable>>>,
    rom_bank: Rc<RefCell<Bank>>,
    rom_bank_value: u8,
    ram_bank_value: u8,
    bank_mode: bool,
    ram_enable: bool,
}

impl Mbc1 {
    pub fn new(header: &Header, rom: Rc<RefCell<Memory>>) -> Result<Self> {
        let mut bus = Bus::default();

        let ram_bank = if header.ram_bank_count()? == 0 {
            None
        } else {
            Some(bank(
                Rc::new(RefCell::new(Memory::new(
                    header.ram_byte_size()?,
                    Access::ReadWrite,
                ))),
                header.ram_bank_size()?,
                header.ram_bank_count()?,
            ))
        };

        let ram_map = if let Some(b) = &ram_bank {
            Some(map_to(
                Rc::clone(b) as Rc<RefCell<dyn Addressable>>,
                RAM_BEGIN..RAM_END,
                RAM_SIZE as u16,
            ))
        } else {
            None
        };

        let rom_bank = bank(
            Rc::clone(&rom) as Rc<RefCell<dyn Addressable>>,
            ROM_BANKX0_SIZE,
            header.rom_bank_count()?,
        );

        bus.add(map_from(
            Rc::clone(&rom) as Rc<RefCell<dyn Addressable>>,
            ROM_BANKX0_BEGIN..ROM_BANKX0_END,
            ROM_BANKX0_SIZE as u16,
        ));

        bus.add(map_to(
            Rc::clone(&rom_bank) as Rc<RefCell<dyn Addressable>>,
            ROM_BANKXX_BEGIN..ROM_BANKXX_END,
            ROM_BANKXX_SIZE as u16,
        ));

        Ok(Self {
            bus,
            ram_bank,
            ram_map,
            rom_bank,
            rom_bank_value: 0,
            ram_bank_value: 0,
            bank_mode: false,
            ram_enable: false,
        })
    }

    fn rebank(&mut self) {
        if self.bank_mode {
            self.rom_bank.borrow_mut().select(
                ((((self.ram_bank_value as u16) & 3) << 4) | ((self.rom_bank_value as u16) & 0xf))
                    as u32,
            );
        } else {
            self.rom_bank.borrow_mut().select(0);
        }

        if let Some(ram_bank) = self.ram_bank.as_ref() {
            ram_bank.borrow_mut().select(
                ((((self.ram_bank_value as u16) & 3) << 4) | ((self.rom_bank_value as u16) & 0xf))
                    as u32,
            );
        }
    }
}

impl Addressable for Mbc1 {
    fn read(&mut self, addr: u16) -> Option<u8> {
        match addr {
            RAM_BEGIN..RAM_END if self.ram_enable && self.ram_map.is_some() => {
                return self.ram_map.as_ref().unwrap().borrow_mut().read(addr)
            }
            _ => self.bus.read(addr),
        }
    }

    fn write(&mut self, addr: u16, value: u8) -> Option<()> {
        match addr {
            ROM_BANK_BEGIN..ROM_BANK_END => {
                self.rom_bank_value = std::cmp::min(1, value);
                self.rebank();
            }
            RAM_ENABLE_BEGIN..RAM_ENABLE_END if value == 0x0a => self.ram_enable = true,
            RAM_ENABLE_BEGIN..RAM_ENABLE_END if value == 0x00 => self.ram_enable = false,
            RAM_BEGIN..RAM_END if self.ram_enable && self.ram_map.is_some() => {
                return self
                    .ram_map
                    .as_ref()
                    .unwrap()
                    .borrow_mut()
                    .write(addr, value)
            }
            RAM_BANK_BEGIN..RAM_BANK_END => {
                self.ram_bank_value = value;
                self.rebank();
            }
            BANK_MODE_BEGIN..BANK_MODE_END => {
                self.bank_mode = (value & 1) != 0;
                self.rebank();
            }
            _ => return self.bus.write(addr, value),
        }

        Some(())
    }
}
