use crate::{
    bus::Bus,
    error::{Error, Result},
};

pub struct Palette {}

impl Bus for Palette {
    fn read(&mut self, addr: u16) -> Result<u8> {}

    fn write(&mut self, addr: u16, value: u8) -> Result<()> {}
}
