use crate::ppu::Ppu;
use std::{cell::RefCell, rc::Rc};

pub struct Clock {
    ppu: Rc<RefCell<Ppu>>,
}

impl Clock {
    pub fn new(ppu: Rc<RefCell<Ppu>>) -> Self {
        Self { ppu }
    }

    pub fn clock(&mut self, m_cycles: u32) {
        let mut ppu = self.ppu.borrow_mut();

        for _ in 0..m_cycles * 4 {
            ppu.dot();
        }
    }
}
