use crate::{
    bus::Bus,
    cartridge::Cartridge,
    io::IO,
    memory::{BankedMemory, Memory},
};
use std::{cell::RefCell, rc::Rc};

pub const VRAM_BEGIN: u16 = 0x8000;
pub const VRAM_END: u16 = 0xa000;
pub const VRAM_SIZE: usize = (VRAM_END - VRAM_BEGIN) as usize;
pub const WRAM_0_BEGIN: u16 = 0xc000;
pub const WRAM_0_END: u16 = 0xe000;
pub const WRAM_X_BEGIN: u16 = 0xc000;
pub const WRAM_X_END: u16 = 0xe000;
pub const WRAM_SIZE: usize = (WRAM_X_END - WRAM_X_BEGIN) as usize;
pub const HRAM_BEGIN: u16 = 0xff80;
pub const HRAM_END: u16 = 0xffff;
pub const HRAM_SIZE: usize = (HRAM_END - HRAM_BEGIN) as usize;

pub struct System {
    cartridge: Cartridge,
    bus: Bus,
}

impl System {
    pub fn new(cartridge: Cartridge) -> Self {
        let bus = Bus::default();

        let vram = Rc::new(RefCell::new(Memory::new(
            VRAM_BEGIN, VRAM_END, VRAM_SIZE, true, true,
        )));

        let wram_0 = Rc::new(RefCell::new(Memory::new(
            WRAM_0_BEGIN,
            WRAM_0_END,
            WRAM_SIZE,
            true,
            true,
        )));

        let wram_x = Rc::new(RefCell::new(BankedMemory::new(
            WRAM_X_BEGIN,
            WRAM_X_END,
            WRAM_SIZE,
            7,
            true,
            true,
        )));

        let hram = Rc::new(RefCell::new(Memory::new(
            HRAM_BEGIN, HRAM_END, HRAM_SIZE, true, true,
        )));

        let io = Rc::new(RefCell::new(IO::new(wram_x)));

        bus.add(vram);
        bus.add(wram_0);
        bus.add(wram_x);
        bus.add(hram);
        bus.add(io);

        Self { cartridge, bus }
    }
}
