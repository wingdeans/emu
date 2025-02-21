use crate::{
    bus::{bank, map_to, Addressable, Bus},
    cartridge::Cartridge,
    io::IO,
    memory::{Access, Memory},
    palette::Palette,
};
use std::{cell::RefCell, rc::Rc};

pub const VRAM_BEGIN: u16 = 0x8000;
pub const VRAM_END: u16 = 0xa000;
pub const VRAM_SIZE: usize = (VRAM_END - VRAM_BEGIN) as usize;
pub const WRAM_0_BEGIN: u16 = 0xc000;
pub const WRAM_0_END: u16 = 0xe000;
pub const WRAM_X_BEGIN: u16 = 0xc000;
pub const WRAM_X_END: u16 = 0xe000;
pub const WRAM_BANK_SIZE: usize = (WRAM_X_END - WRAM_X_BEGIN) as usize;
pub const WRAM_BANK_COUNT: u32 = 8;
pub const HRAM_BEGIN: u16 = 0xff80;
pub const HRAM_END: u16 = 0xffff;
pub const HRAM_SIZE: usize = (HRAM_END - HRAM_BEGIN) as usize;

pub struct System {
    bus: Bus,
    palette: Rc<RefCell<Palette>>,
}

impl System {
    pub fn new(cartridge: Cartridge) -> Self {
        let mut bus = Bus::default();

        let vram = map_to(
            Rc::new(RefCell::new(Memory::new(VRAM_SIZE, Access::ReadWrite))),
            VRAM_BEGIN..VRAM_END,
            VRAM_SIZE as u16,
        );

        let wram = bank(
            Rc::new(RefCell::new(Memory::new(
                WRAM_BANK_SIZE * WRAM_BANK_COUNT as usize,
                Access::ReadWrite,
            ))),
            WRAM_BANK_SIZE,
            WRAM_BANK_COUNT,
        );

        let wram_0 = Rc::clone(wram.borrow().bank(0));

        let hram = map_to(
            Rc::new(RefCell::new(Memory::new(HRAM_SIZE, Access::ReadWrite))),
            HRAM_BEGIN..HRAM_END,
            HRAM_SIZE as u16,
        );

        let io = Rc::new(RefCell::new(IO::new(Rc::clone(&wram))));
        let palette: Rc<RefCell<Palette>> = Default::default();

        bus.add(Rc::new(RefCell::new(cartridge)));
        bus.add(vram);
        bus.add(wram_0);
        bus.add(wram);
        bus.add(hram);
        bus.add(io);
        bus.add(Rc::clone(&palette) as Rc<RefCell<dyn Addressable>>);

        Self { bus, palette }
    }

    pub fn bus_mut(&mut self) -> &mut Bus {
        &mut self.bus
    }

    pub fn palette_ref(&self) -> &Rc<RefCell<Palette>> {
        &self.palette
    }
}

impl Addressable for System {
    fn read(&mut self, addr: u16) -> Option<u8> {
        self.bus.read(addr)
    }

    fn write(&mut self, addr: u16, value: u8) -> Option<()> {
        self.bus.write(addr, value)
    }
}
