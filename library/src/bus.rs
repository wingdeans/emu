use crate::error::Result;

pub trait Bus {
    fn read(&mut self, addr: u16) -> Result<u8>;
    fn write(&mut self, addr: u16, value: u8) -> Result<()>;
}
