use crate::bus::Addressable;
use std::{cell::RefCell, rc::Rc};

pub const IE_ADDRESS: u16 = 0xffff;
pub const IF_ADDRESS: u16 = 0xff0f;

pub const VBLANK_INT_FLAG: u8 = 0;

pub trait InterruptHandler {
    fn ime(&self) -> bool;
    fn handle(&mut self, pc: u16);
}

pub struct Interrupt {
    enable: u8,
    flag: u8,
    handler: Rc<RefCell<dyn InterruptHandler>>,
}

impl Interrupt {
    pub fn new(handler: Rc<RefCell<dyn InterruptHandler>>) -> Self {
        Self {
            enable: 0,
            flag: 0xe1,
            handler,
        }
    }
}

impl Addressable for Interrupt {
    fn read(&mut self, addr: u16) -> Option<u8> {
        match addr {
            IE_ADDRESS => Some(self.enable),
            IF_ADDRESS => Some(self.flag),
            _ => None,
        }
    }

    fn write(&mut self, addr: u16, value: u8) -> Option<()> {
        match addr {
            IE_ADDRESS => self.enable = value,
            IF_ADDRESS => {
                self.flag = value;
                let mut mask = self.enable & self.flag;

                if mask != 0 && self.handler.borrow().ime() {
                    for i in 0..5 {
                        if mask & (1 << i) != 0 {
                            mask ^= 1 << i;
                            self.handler.borrow_mut().handle(0x40 + i * 8);
                        }
                    }
                }
            }
            _ => return None,
        }

        Some(())
    }
}
