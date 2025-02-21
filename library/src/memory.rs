use crate::bus::Addressable;

#[derive(Clone)]
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
    access: Access,
    data: Vec<u8>,
}

impl Memory {
    pub fn new(size: usize, access: Access) -> Self {
        Memory {
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

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl Addressable for Memory {
    fn read(&mut self, addr: u16) -> Option<u8> {
        if self.access.readable() && (addr as usize) < self.data.len() {
            Some(self.data[addr as usize])
        } else {
            None
        }
    }

    fn write(&mut self, addr: u16, value: u8) -> Option<()> {
        if self.access.writable() && (addr as usize) < self.data.len() {
            self.data[addr as usize] = value;
            Some(())
        } else {
            None
        }
    }
}
