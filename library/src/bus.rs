use std::ops::Range;
use std::{cell::RefCell, rc::Rc};

pub trait Addressable {
    fn read(&mut self, addr: u16) -> Option<u8>;
    fn write(&mut self, addr: u16, value: u8) -> Option<()>;
}

pub trait Mapped: Addressable {
    fn range(&self) -> &Range<u16>;
    fn size(&self) -> usize;
}

pub struct View {
    range: Range<u16>,
    mapped: Rc<RefCell<dyn Mapped>>,
}

impl View {
    pub fn of(mapped: Rc<RefCell<dyn Mapped>>, range: Range<u16>) -> Rc<RefCell<dyn Addressable>> {
        Rc::new(RefCell::new(Self { range, mapped }))
    }

    fn remap(&self, addr: u16) -> u16 {
        let a = self.mapped.borrow();
        ((addr - self.range.start) % (a.size() as u16)) + a.range().start
    }
}

impl Addressable for View {
    fn read(&mut self, addr: u16) -> Option<u8> {
        if self.range.contains(&addr) {
            let a = self.remap(addr);
            self.mapped.borrow_mut().read(a)
        } else {
            None
        }
    }

    fn write(&mut self, addr: u16, value: u8) -> Option<()> {
        if self.range.contains(&addr) {
            let a = self.remap(addr);
            self.mapped.borrow_mut().write(a, value)
        } else {
            None
        }
    }
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
