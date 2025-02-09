use library::{bus::Bus, error::Result};
use std::{cell::RefCell, rc::Rc};

pub struct Memory {
    data: [u8; 0xffff],
}

impl Memory {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { data: [0; 0xffff] }))
    }
}

impl Bus for Memory {
    fn read(&mut self, addr: u16) -> Result<u8> {
        Ok(self.data[addr as usize])
    }

    fn write(&mut self, addr: u16, value: u8) -> Result<()> {
        self.data[addr as usize] = value;
        Ok(())
    }
}
