use crate::bus::Addressable;
use std::{cell::RefCell, rc::Rc};

pub const IE_ADDRESS: u16 = 0xffff;
pub const IF_ADDRESS: u16 = 0xff0f;

pub const VBLANK_INT_FLAG: u8 = 1;
pub const STAT_INT_FLAG: u8 = 2;
pub const TIMA_INT_FLAG: u8 = 4;
pub const JOYPAD_INT_FLAG: u8 = 16;

pub trait InterruptHandler {
    fn ime(&self) -> bool;
    fn handle(&mut self, pc: u16);
}

pub struct Interrupt {
    pub enable: u8,
    pub flag: u8,
    pub handler: Rc<RefCell<dyn InterruptHandler>>,
}

impl Interrupt {
    pub fn new(handler: Rc<RefCell<dyn InterruptHandler>>) -> Self {
        Self {
            enable: 0,
            flag: 0xe1,
            handler,
        }
    }

    pub fn set(&mut self, mask: u8) {
        self.flag |= mask;
    }

    pub fn unset(&mut self, mask: u8) {
        self.flag = self.flag & !mask;
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
            IE_ADDRESS => {
                self.enable = value;

                if self.enable == 0 {
                    panic!();
                }
            }
            IF_ADDRESS => self.flag = value,
            _ => return None,
        }

        Some(())
    }
}
