use crate::{
    bus::Addressable,
    surface::{Surface, SCREEN_WIDTH},
};
use std::{cell::RefCell, cmp::min, rc::Rc};

pub const LCD_Y_ADDRESS: u16 = 0xff44;

const DOTS_IN_LINE: u32 = 80 + 376;

pub struct Ppu {
    surface: Rc<RefCell<dyn Surface>>,
    dots: u32,
    x: u8,
    y: u8,
}

impl Ppu {
    pub fn new(surface: Rc<RefCell<dyn Surface>>) -> Self {
        Self {
            surface,
            dots: 0,
            x: 0,
            y: 0,
        }
    }

    pub fn dot(&mut self) {
        self.dots += 1;
        let line = self.dots % DOTS_IN_LINE;

        self.x = min(line - min(line, 80), SCREEN_WIDTH - 1) as u8;

        if self.dots % DOTS_IN_LINE == 0 {
            self.y += 1;
            self.surface.borrow_mut().flush();
        }
    }

    pub fn get_x(&self) -> u8 {
        self.x
    }

    pub fn get_y(&self) -> u8 {
        self.y
    }
}

impl Addressable for Ppu {
    fn read(&mut self, addr: u16) -> Option<u8> {
        match addr {
            LCD_Y_ADDRESS => Some(self.y),
            _ => None,
        }
    }

    fn write(&mut self, addr: u16, value: u8) -> Option<()> {
        None
    }
}
