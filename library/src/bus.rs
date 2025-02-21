use std::ops::Range;
use std::{cell::RefCell, rc::Rc};

pub trait Addressable {
    fn read(&mut self, addr: u16) -> Option<u8>;
    fn write(&mut self, addr: u16, value: u8) -> Option<()>;
}

struct Map {
    elem: Rc<RefCell<dyn Addressable>>,
    from: Range<u16>,
    to: Range<u16>,
}

pub fn map_to(
    elem: Rc<RefCell<dyn Addressable>>,
    range: Range<u16>,
    size: u16,
) -> Rc<RefCell<dyn Addressable>> {
    Rc::new(RefCell::new(Map {
        elem,
        from: 0..size,
        to: range,
    }))
}

pub fn map_from(
    elem: Rc<RefCell<dyn Addressable>>,
    range: Range<u16>,
    size: u16,
) -> Rc<RefCell<dyn Addressable>> {
    Rc::new(RefCell::new(Map {
        elem,
        from: range,
        to: 0..size,
    }))
}

pub fn map(
    elem: Rc<RefCell<dyn Addressable>>,
    from: Range<u16>,
    to: Range<u16>,
) -> Rc<RefCell<dyn Addressable>> {
    Rc::new(RefCell::new(Map { elem, from, to }))
}

impl Addressable for Map {
    fn read(&mut self, addr: u16) -> Option<u8> {
        if self.to.contains(&addr) {
            let address =
                ((addr - self.to.start) + self.from.start) % (self.from.end - self.from.start);
            self.elem.borrow_mut().read(address)
        } else {
            None
        }
    }

    fn write(&mut self, addr: u16, value: u8) -> Option<()> {
        if self.to.contains(&addr) {
            let address =
                ((addr - self.to.start) + self.from.start) % (self.from.end - self.from.start);
            self.elem.borrow_mut().write(address, value)
        } else {
            None
        }
    }
}

pub struct Bank {
    index: u32,
    banks: Vec<Rc<RefCell<dyn Addressable>>>,
}

pub fn bank(
    elem: Rc<RefCell<dyn Addressable>>,
    bank_size: usize,
    bank_count: u32,
) -> Rc<RefCell<Bank>> {
    let mut banks = Vec::<Rc<RefCell<dyn Addressable>>>::with_capacity(bank_count as usize);

    for i in 0..bank_count as usize {
        let range = (i * bank_size) as u16..((i + 1) * bank_size) as u16;
        banks.push(map_from(Rc::clone(&elem), range, bank_size as u16));
    }

    Rc::new(RefCell::new(Bank { index: 0, banks }))
}

impl Bank {
    pub fn select(&mut self, bank: u32) {
        self.index = bank.clamp(0, self.banks.len() as u32);
    }

    pub fn selected(&self) -> u32 {
        self.index
    }

    pub fn bank(&self, index: usize) -> &Rc<RefCell<dyn Addressable>> {
        &self.banks[index]
    }
}

impl Addressable for Bank {
    fn read(&mut self, addr: u16) -> Option<u8> {
        self.banks[self.index as usize].borrow_mut().read(addr)
    }

    fn write(&mut self, addr: u16, value: u8) -> Option<()> {
        self.banks[self.index as usize]
            .borrow_mut()
            .write(addr, value)
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
        self.components
            .iter()
            .find_map(|comp| comp.borrow_mut().read(addr))
    }

    fn write(&mut self, addr: u16, value: u8) -> Option<()> {
        self.components
            .iter()
            .find_map(|comp| comp.borrow_mut().write(addr, value))
    }
}
