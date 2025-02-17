use crate::{bus::Bus, cartridge::Cartridge, error::Result, io::IO, wram::Wram};
use std::{cell::RefCell, rc::Rc};

pub const VRAM_BEGIN: u16 = 0x8000;
pub const VRAM_END: u16 = 0xa000;
pub const VRAM_SIZE: usize = (VRAM_END - VRAM_BEGIN) as usize;
pub const WRAM_BEGIN: u16 = 0xc000;
pub const WRAM_END: u16 = 0xe000;
pub const WRAM_SIZE: usize = (WRAM_END - WRAM_BEGIN) as usize;
pub const IO_BEGIN: u16 = 0xff00;
pub const IO_END: u16 = 0xff7f;

pub struct System {
    cartridge: Cartridge,
    vram: Vec<u8>,
    wram: Rc<RefCell<Wram>>,
    io: IO,
}

impl System {
    pub fn new(cartridge: Cartridge) -> Self {
        let wram: Rc<RefCell<Wram>> = Default::default();

        Self {
            cartridge,
            vram: vec![0u8; VRAM_SIZE],
            wram: Rc::clone(&wram),
            io: IO::new(wram),
        }
    }
}

impl Bus for System {
    fn read(&mut self, addr: u16) -> Result<u8> {
        match addr {
            VRAM_BEGIN..VRAM_END => Ok(self.vram[(addr & 0x1ff) as usize]),
            WRAM_BEGIN..WRAM_END => self.wram.borrow_mut().read(addr),
            IO_BEGIN..IO_END => self.io.read(addr),
            _ => self.cartridge.read(addr),
        }
    }

    fn write(&mut self, addr: u16, value: u8) -> Result<()> {
        match addr {
            VRAM_BEGIN..VRAM_END => self.vram[(addr & 0x1ff) as usize] = value,
            WRAM_BEGIN..WRAM_END => self.wram.borrow_mut().write(addr, value)?,
            IO_BEGIN..IO_END => self.io.write(addr, value)?,
            _ => self.cartridge.write(addr, value)?,
        }

        Ok(())
    }
}
