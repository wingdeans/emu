use crate::bus::{Addressable, Bank};
use std::{cell::RefCell, rc::Rc};

pub const WRAM_BANK_SELECT: u16 = 0xff70;

pub struct IO {
    wram: Rc<RefCell<Bank>>,
}

impl IO {
    pub fn new(wram: Rc<RefCell<Bank>>) -> Self {
        Self { wram }
    }
}

impl Addressable for IO {
    fn read(&mut self, addr: u16) -> Option<u8> {
        match addr {
            WRAM_BANK_SELECT => Some(self.wram.borrow().selected() as u8),
            _ => None,
        }
    }

    fn write(&mut self, addr: u16, value: u8) -> Option<()> {
        match addr {
            WRAM_BANK_SELECT => self
                .wram
                .borrow_mut()
                .select(std::cmp::max(1, value as u32)),
            _ => return None,
        }

        Some(())
    }
}
