use crate::{
    bus::{Addressable, Bus, Mapped, View},
    cartridge::Header,
    error::Result,
    memory::{Access, Memory, MemoryBank},
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
    pub fn new(header: &Header, rom: Rc<RefCell<MemoryBank>>) -> Result<Self> {
        let ram = Rc::new(RefCell::new(Memory::new(
            RAM_BEGIN..RAM_END,
            header.ram_byte_size()?,
            Access::ReadWrite,
        )));

        let mut bus = Bus::default();
        bus.add(View::of(rom as Rc<RefCell<dyn Mapped>>, ROM_BEGIN..ROM_END));
        bus.add(ram);

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
