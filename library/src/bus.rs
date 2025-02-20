use std::{cell::RefCell, rc::Rc};

pub trait Addressable {
    fn read(&mut self, addr: u16) -> Option<u8>;
    fn write(&mut self, addr: u16, value: u8) -> Option<()>;
}

#[derive(Default)]
pub struct Bus {
    components: Vec<Rc<RefCell<dyn Addressable>>>,
}

impl Bus {
    pub fn add(&mut self, comp: Rc<RefCell<dyn Addressable>>) {
        self.components.push(comp);
    }
}

impl Addressable for Bus {
    fn read(&mut self, addr: u16) -> Option<u8> {
        for comp in self.components.iter() {
            return comp.borrow_mut().read(addr);
        }

        return None;
    }

    fn write(&mut self, addr: u16, value: u8) -> Option<()> {
        for comp in self.components.iter() {
            return comp.borrow_mut().write(addr, value);
        }

        return None;
    }
}
