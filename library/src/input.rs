use crate::bus::Addressable;
use std::{cell::RefCell, rc::Rc};

pub const JOYP_ADDRESS: u16 = 0xff00;

pub trait InputHandler {
    fn start(&self) -> bool;
    fn select(&self) -> bool;
    fn b(&self) -> bool;
    fn a(&self) -> bool;

    fn up(&self) -> bool;
    fn down(&self) -> bool;
    fn left(&self) -> bool;
    fn right(&self) -> bool;
}

pub struct Input {
    handler: Rc<RefCell<dyn InputHandler>>,
    mode: u8,
}

impl Input {
    pub fn new(handler: Rc<RefCell<dyn InputHandler>>) -> Self {
        Self { handler, mode: 3 }
    }

    fn get_select_buttons(&self) -> u8 {
        let start = if self.handler.borrow().start() { 0 } else { 1 };
        let select = if self.handler.borrow().select() { 0 } else { 1 };
        let b = if self.handler.borrow().b() { 0 } else { 1 };
        let a = if self.handler.borrow().a() { 0 } else { 1 };

        start << 3 | select << 2 | b << 1 | a
    }

    fn get_dpad_buttons(&self) -> u8 {
        let up = if self.handler.borrow().up() { 0 } else { 1 };
        let down = if self.handler.borrow().down() { 0 } else { 1 };
        let left = if self.handler.borrow().left() { 0 } else { 1 };
        let right = if self.handler.borrow().right() { 0 } else { 1 };

        down << 3 | up << 2 | left << 1 | right
    }
}

impl Addressable for Input {
    fn read(&mut self, addr: u16) -> Option<u8> {
        if addr == JOYP_ADDRESS {
            let lo = if self.mode & 1 == 0 {
                self.get_dpad_buttons()
            } else if self.mode & 2 == 0 {
                self.get_select_buttons()
            } else {
                0xf
            };

            Some(self.mode << 4 | lo)
        } else {
            None
        }
    }

    fn write(&mut self, addr: u16, value: u8) -> Option<()> {
        if addr == JOYP_ADDRESS {
            self.mode = value >> 4;
            Some(())
        } else {
            None
        }
    }
}
