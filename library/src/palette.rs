use crate::bus::Addressable;

pub struct Palette {}

impl Addressable for Palette {
    fn read(&mut self, addr: u16) -> Option<u8> {
        None
    }

    fn write(&mut self, addr: u16, value: u8) -> Option<()> {
        None
    }
}
