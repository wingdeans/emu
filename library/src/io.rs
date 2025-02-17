use crate::{
    bus::Bus,
    error::{Error, Result},
    wram::Wram,
};
use std::{cell::RefCell, rc::Rc};

pub const WRAM_BANK_SELECT: u16 = 0xff70;

pub struct IO {
    wram: Rc<RefCell<Wram>>,
}

impl IO {
    pub fn new(wram: Rc<RefCell<Wram>>) -> Self {
        Self { wram }
    }
}

impl Bus for IO {
    fn read(&mut self, addr: u16) -> Result<u8> {
        match addr {
            WRAM_BANK_SELECT => Ok(self.wram.borrow().bank() as u8),
            _ => Err(Error::InvalidIOAccess(addr)),
        }
    }

    fn write(&mut self, addr: u16, value: u8) -> Result<()> {
        match addr {
            WRAM_BANK_SELECT => self.wram.borrow_mut().set_bank(value as u32),
            _ => return Err(Error::InvalidIOAccess(addr)),
        }

        Ok(())
    }
}
