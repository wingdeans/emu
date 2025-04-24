use crate::bus::Addressable;

pub const DATA_ADDR: u16 = 0xff01;
pub const CONTROL_ADDR: u16 = 0xff02;

/// Serial is not supported, but a stub implementation is necessary
#[derive(Default)]
pub struct Serial {
    control: u8,
}

impl Addressable for Serial {
    fn read(&mut self, addr: u16) -> Option<u8> {
        match addr {
            DATA_ADDR => Some(0),
            CONTROL_ADDR => Some(self.control),
            _ => None,
        }
    }

    fn write(&mut self, addr: u16, value: u8) -> Option<()> {
        match addr {
            DATA_ADDR => Some(()),
            CONTROL_ADDR => {
                self.control = value;
                Some(())
            }
            _ => None,
        }
    }
}
