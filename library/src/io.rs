use crate::{
    bus::{Addressable, Bank},
    int::{Interrupt, TIMA_INT_FLAG},
};
use std::{cell::RefCell, rc::Rc};

pub const WRAM_BANK_SELECT: u16 = 0xff70;
pub const VRAM_BANK_SELECT: u16 = 0xff4f;
pub const TIMA_ADDRESS: u16 = 0xff05;
pub const TMA_ADDRESS: u16 = 0xff06;
pub const TAC_ADDRESS: u16 = 0xff07;
pub const DIV_ADDRESS: u16 = 0xff04;

pub struct IO {
    int: Rc<RefCell<Interrupt>>,
    wram: Rc<RefCell<Bank>>,
    vram: Rc<RefCell<Bank>>,
    tima: u8,
    tma: u8,
    div: u8,
    enable_tima: bool,
    tima_clock: u8,
    tima_acc: u32,
}

impl IO {
    pub fn new(
        int: Rc<RefCell<Interrupt>>,
        wram: Rc<RefCell<Bank>>,
        vram: Rc<RefCell<Bank>>,
    ) -> Self {
        Self {
            int,
            wram,
            vram,
            tima: 0,
            tma: 0,
            div: 0,
            enable_tima: false,
            tima_clock: 0,
            tima_acc: 0,
        }
    }

    pub fn clock(&mut self, m_cycles: u32) {
        self.div = self.div.wrapping_add(m_cycles as u8 * 16);

        if self.enable_tima {
            self.tima_acc += m_cycles;

            let boundary = match self.tima_clock & 3 {
                0b00 => 256,
                0b01 => 4000,
                0b10 => 16000000,
                0b11 => 64000000,
                _ => unreachable!(),
            };

            if self.tima_acc > boundary {
                self.tima_acc -= boundary;
                let (value, overflow) = self.tima.overflowing_add(1);

                if overflow {
                    self.tima = self.tma;
                    self.int.borrow_mut().int(TIMA_INT_FLAG);
                } else {
                    self.tima = value;
                }
            }
        }
    }
}

impl Addressable for IO {
    fn read(&mut self, addr: u16) -> Option<u8> {
        match addr {
            WRAM_BANK_SELECT => Some(self.wram.borrow().selected() as u8),
            VRAM_BANK_SELECT => Some(self.vram.borrow().selected() as u8 | !1),
            TIMA_ADDRESS => Some(self.tima),
            TMA_ADDRESS => Some(self.tma),
            DIV_ADDRESS => Some(self.div),
            _ => None,
        }
    }

    fn write(&mut self, addr: u16, value: u8) -> Option<()> {
        match addr {
            WRAM_BANK_SELECT => self
                .wram
                .borrow_mut()
                .select(std::cmp::max(1, value as u32)),
            VRAM_BANK_SELECT => self.vram.borrow_mut().select(value as u32 & 1),
            TIMA_ADDRESS => self.tima = value,
            TMA_ADDRESS => self.tma = value,
            TAC_ADDRESS => {
                self.enable_tima = (value & 4) != 0;
                self.tima_clock = value & 3;
            }
            DIV_ADDRESS => self.div = 0,
            _ => return None,
        }

        Some(())
    }
}
