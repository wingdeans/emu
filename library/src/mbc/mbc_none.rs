use crate::{
    bus::{map_to, Addressable, Bus},
    cartridge::Header,
    error::Result,
    memory::{Access, Memory},
};
use std::{cell::RefCell, rc::Rc};

pub const ROM_BEGIN: u16 = 0x0000;
pub const ROM_END: u16 = 0x8000;
pub const RAM_BEGIN: u16 = 0xa000;
pub const RAM_END: u16 = 0xc000;

pub struct MbcNone {
    bus: Bus,
}

impl MbcNone {
    pub fn new(header: &Header, rom: Rc<RefCell<Memory>>) -> Result<Self> {
        let ram_size = header.ram_byte_size()?;
        let rom_size = header.rom_byte_size()?;

        let mut bus = Bus::default();
        bus.add(map_to(rom, ROM_BEGIN..ROM_END, rom_size as u16));

        if ram_size != 0 {
            let ram = Rc::new(RefCell::new(Memory::new(
                header.ram_byte_size()?,
                Access::ReadWrite,
            )));

            bus.add(map_to(ram, RAM_BEGIN..RAM_END, ram_size as u16));
        }

        Ok(Self { bus })
    }
}

impl Addressable for MbcNone {
    fn read(&mut self, addr: u16) -> Option<u8> {
        self.bus.read(addr)
    }

    fn write(&mut self, addr: u16, value: u8) -> Option<()> {
        self.bus.write(addr, value)
    }
}
