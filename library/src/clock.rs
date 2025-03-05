use crate::ppu::Ppu;
use std::{cell::RefCell, rc::Rc};

const CYCLES_PER_SCANLINE: u32 = (80 + 376) / 4;

pub struct Clock {
    ppu: Rc<RefCell<Ppu>>,
    cycles: u32,
}

impl Clock {
    pub fn new(ppu: Rc<RefCell<Ppu>>) -> Self {
        Self { ppu, cycles: 0 }
    }

    pub fn clock(&mut self, m_cycles: u32) {
        if self.cycles % CYCLES_PER_SCANLINE > (self.cycles + m_cycles) % CYCLES_PER_SCANLINE {
            self.ppu.borrow_mut().scanline();
        }

        self.cycles += m_cycles;
    }
}
