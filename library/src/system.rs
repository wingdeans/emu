use crate::{bus::Bus, cartridge::Cartridge, error::Result};
pub const VRAM_BEGIN: u16 = 0x8000;
pub const VRAM_END: u16 = 0xa000;
pub const VRAM_SIZE: usize = (VRAM_END - VRAM_BEGIN) as usize;

pub struct System {
    cartridge: Cartridge,
    vram: Vec<u8>,
}

impl System {
    pub fn new(cartridge: Cartridge) -> Self {
        Self {
            cartridge,
            vram: vec![0u8; VRAM_SIZE as usize],
        }
    }
}

impl Bus for System {
    fn read(&mut self, addr: u16) -> Result<u8> {
        match addr {
            VRAM_BEGIN..VRAM_END => Ok(self.vram[(addr & 0x1ff) as usize]),
            _ => self.cartridge.read(addr),
        }
    }

    fn write(&mut self, addr: u16, value: u8) -> Result<()> {
        match addr {
            VRAM_BEGIN..VRAM_END => self.vram[(addr & 0x1ff) as usize] = value,
            _ => self.cartridge.write(addr, value)?,
        }

        Ok(())
    }
}
