use crate::bus::Addressable;
use std::{cell::RefCell, rc::Rc};

pub const HDMA1_ADDRESS: u16 = 0xff51;
pub const HDMA2_ADDRESS: u16 = 0xff52;
pub const HDMA3_ADDRESS: u16 = 0xff53;
pub const HDMA4_ADDRESS: u16 = 0xff54;
pub const HDMA5_ADDRESS: u16 = 0xff55;
pub const OAM_DMA_ADDRESS: u16 = 0xff46;

pub struct Dma {
    bus: Rc<RefCell<dyn Addressable>>,
    dma_src: u16,
    dma_dst: u16,
    dma_length: usize,
    dma_result: u8,
}

impl Dma {
    pub fn new(bus: Rc<RefCell<dyn Addressable>>) -> Self {
        Self {
            bus,
            dma_src: 0,
            dma_dst: 0,
            dma_length: 0,
            dma_result: 0xff,
        }
    }

    pub fn scanline(&mut self) {
        if self.dma_length == 0 {
            return;
        }

        let mut bus = self.bus.borrow_mut();

        for i in 0..0x10u16 {
            bus.read(self.dma_src + i)
                .map(|value| bus.write(self.dma_dst + i, value));
        }

        self.dma_length -= 0x10;
        self.dma_src += 0x10;
        self.dma_dst += 0x10;
    }

    fn handle(&mut self, mode: u8) {
        if self.dma_length != 0 && mode & 0x80 != 0 {
            self.dma_result = (self.dma_length / 0x10 - 1) as u8 + 0x80;
        }

        self.dma_length = (((mode & 0x7f) + 1) * 0x10) as usize;

        if mode & 0x80 == 0 {
            let mut bus = self.bus.borrow_mut();

            for i in 0..self.dma_length as u16 {
                bus.read(self.dma_src + i)
                    .map(|value| bus.write(self.dma_dst + i, value));
            }

            self.dma_length = 0;
            self.dma_result = 0xff;
        } else {
            self.dma_result = (self.dma_length / 0x10 - 1) as u8;
        }
    }

    pub fn oam(&mut self, addr: u8) {
        let mut bus = self.bus.borrow_mut();

        for i in 0..0xa0 {
            bus.read(((addr as u16) << 8) | i)
                .map(|value| bus.write(0xfe00 | i, value));
        }
    }
}

impl Addressable for Dma {
    fn read(&mut self, addr: u16) -> Option<u8> {
        match addr {
            HDMA5_ADDRESS => Some(self.dma_result),
            _ => None,
        }
    }

    fn write(&mut self, addr: u16, value: u8) -> Option<()> {
        match addr {
            HDMA1_ADDRESS => self.dma_src = ((value as u16) << 8) | (self.dma_src & 0xff),
            HDMA2_ADDRESS => self.dma_src = (self.dma_src & 0xff00) | (value as u16 & 0xf0),
            HDMA3_ADDRESS => self.dma_dst = ((value as u16) << 8) | (self.dma_dst & 0xff),
            HDMA4_ADDRESS => self.dma_dst = (self.dma_dst & 0xff00) | (value as u16 & 0xf0),
            HDMA5_ADDRESS => self.handle(value),
            OAM_DMA_ADDRESS => self.oam(value),
            _ => return None,
        }

        Some(())
    }
}
