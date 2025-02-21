use crate::bus::{Addressable, Mapped};
use std::ops::Range;
use std::{cell::RefCell, rc::Rc};

pub enum Access {
    ReadOnly,
    WriteOnly,
    ReadWrite,
}

impl Access {
    pub fn readable(&self) -> bool {
        match self {
            Self::ReadOnly | Self::ReadWrite => true,
            _ => false,
        }
    }

    pub fn writable(&self) -> bool {
        match self {
            Self::WriteOnly | Self::ReadWrite => true,
            _ => false,
        }
    }
}

pub struct Memory {
    range: Range<u16>,
    access: Access,
    data: Vec<u8>,
}

impl Memory {
    pub fn new(range: Range<u16>, size: usize, access: Access) -> Self {
        Memory {
            range,
            access,
            data: vec![0u8; size],
        }
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }
}

impl Addressable for Memory {
    fn read(&mut self, addr: u16) -> Option<u8> {
        if self.access.readable() && self.data.len() != 0 && self.range.contains(&addr) {
            Some(self.data[((addr - self.range.start) as usize) % self.data.len()])
        } else {
            None
        }
    }

    fn write(&mut self, addr: u16, value: u8) -> Option<()> {
        if self.access.writable() && self.data.len() != 0 && self.range.contains(&addr) {
            let a = ((addr - self.range.start) as usize) % self.data.len();
            self.data[a] = value;
            Some(())
        } else {
            None
        }
    }
}

impl Mapped for Memory {
    fn range(&self) -> &Range<u16> {
        &self.range
    }

    fn size(&self) -> usize {
        self.data.len()
    }
}

pub struct MemoryBank {
    range: Range<u16>,
    bank_range: Range<u8>,
    size: usize,
    index: u32,
    banks: Vec<Rc<RefCell<Memory>>>,
}

impl MemoryBank {
    pub fn new(range: Range<u16>, bank_size: usize, bank_range: Range<u8>, access: Access) -> Self {
        let banks = vec![
            Rc::new(RefCell::new(Memory::new(range.clone(), bank_size, access)));
            (bank_range.end - bank_range.start) as usize
        ];

        Self {
            range,
            bank_range,
            size: bank_size,
            index: 0,
            banks,
        }
    }

    pub fn bank(&self, bank: u32) -> &Rc<RefCell<Memory>> {
        &self.banks[bank as usize]
    }

    pub fn select(&mut self, bank: u32) {
        self.index = bank.clamp(self.bank_range.start as u32, self.bank_range.end as u32)
            - (self.bank_range.start as u32);
    }

    pub fn selected(&self) -> u32 {
        self.index
    }
}

impl Addressable for MemoryBank {
    fn read(&mut self, addr: u16) -> Option<u8> {
        self.banks[self.index as usize].borrow_mut().read(addr)
    }

    fn write(&mut self, addr: u16, value: u8) -> Option<()> {
        self.banks[self.index as usize]
            .borrow_mut()
            .write(addr, value)
    }
}

impl Mapped for MemoryBank {
    fn range(&self) -> &Range<u16> {
        &self.range
    }

    fn size(&self) -> usize {
        self.size
    }
}
