use crate::bus::Addressable;

pub struct Memory {
    begin: u16,
    end: u16,
    readable: bool,
    writable: bool,
    mem: Vec<u8>,
}

impl Memory {
    pub fn new(begin: u16, end: u16, size: usize, readable: bool, writable: bool) -> Self {
        Memory {
            begin,
            end,
            readable,
            writable,
            mem: vec![0u8; size],
        }
    }

    pub fn range(&self) -> (u16, u16) {
        (self.begin, self.end)
    }
}

impl Addressable for Memory {
    fn read(&mut self, addr: u16) -> Option<u8> {
        if self.readable && self.mem.len() > 0 && addr >= self.begin && addr < self.end {
            Some(Ok(self.mem[((addr - self.begin) as usize) % self.mem.len()]));
        } else {
            None
        }
    }

    fn write(&mut self, addr: u16, value: u8) -> Option<()> {
        if self.writable && self.mem.len() > 0 && addr >= self.begin && addr < self.end {
            self.mem[((addr - self.begin) as usize) % self.mem.len()] = value;
            Some(())
        } else {
            None
        }
    }
}

pub struct MemoryView {
    begin: u16,
    end: u16,
    memory: Rc<RefCell<Memory>>,
}

impl Addressable for MemoryView {}

pub struct MemoryBank {
    begin: u16,
    end: u16,
    index: u32,
    banks: Vec<Memory>,
}

impl MemoryBank {
    pub fn new(
        begin: u16,
        end: u16,
        bank_size: usize,
        bank_count: u32,
        readable: bool,
        writable: bool,
    ) -> Self {
        Self {
            begin,
            end,
            index: 0,
            banks: vec![Memory::new(begin, end, bank_size, readable, writable); bank_count],
        }
    }

    pub fn bank(&mut self, bank: u32) -> &mut Memory {
        self.banks[bank as usize]
    }

    pub fn select(&mut self, bank: u32) {
        self.index = std::cmp::min(bank, self.banks.len());
    }

    pub fn selected(&self) -> u32 {
        self.index
    }
}

impl Addressable for MemoryBank {
    fn read(&mut self, addr: u16) -> Option<u8> {
        self.banks[self.index].read(addr)
    }

    fn write(&mut self, addr: u16, value: u8) -> Option<()> {
        self.banks[self.index].write(addr, value)
    }
}
