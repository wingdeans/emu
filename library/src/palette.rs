use crate::bus::Addressable;

pub struct Color(pub u8, pub u8, pub u8);

pub enum ColorID {
    ID0,
    ID1,
    ID2,
    ID3,
}

pub const BGP_ADDRESS: u16 = 0xff47;
pub const OBP0_ADDRESS: u16 = 0xff48;
pub const OBP1_ADDRESS: u16 = 0xff49;
pub const BCPS_BGPI_ADDRESS: u16 = 0xff68;

#[derive(Default)]
pub struct Palette {
    bgp: u8,
    obp0: u8,
    obp1: u8,
}

impl Palette {
    fn decode_monochrome_color_id(id: u8) -> Color {
        match id {
            0 => Color(0xff, 0xff, 0xff),
            1 => Color(0xa9, 0xa9, 0xa9),
            2 => Color(0x54, 0x54, 0x54),
            3 => Color(0x00, 0x00, 0x00),
            _ => unreachable!(),
        }
    }

    fn decode_monochrome_color(id: ColorID, value: u8) -> Color {
        use ColorID::*;

        match id {
            ID0 => Self::decode_monochrome_color_id(value & 0b11),
            ID1 => Self::decode_monochrome_color_id((value >> 2) & 0b11),
            ID2 => Self::decode_monochrome_color_id((value >> 4) & 0b11),
            ID3 => Self::decode_monochrome_color_id((value >> 6) & 0b11),
        }
    }

    pub fn get_bgp(&self, id: ColorID) -> Color {
        Self::decode_monochrome_color(id, self.bgp)
    }

    pub fn get_obp0(&self, id: ColorID) -> Color {
        Self::decode_monochrome_color(id, self.obp0)
    }

    pub fn get_obp1(&self, id: ColorID) -> Color {
        Self::decode_monochrome_color(id, self.obp1)
    }
}

impl Addressable for Palette {
    fn read(&mut self, addr: u16) -> Option<u8> {
        match addr {
            BGP_ADDRESS => Some(self.bgp),
            OBP0_ADDRESS => Some(self.obp0),
            OBP1_ADDRESS => Some(self.obp1),
            _ => None,
        }
    }

    fn write(&mut self, addr: u16, value: u8) -> Option<()> {
        match addr {
            BGP_ADDRESS => self.bgp = value,
            OBP0_ADDRESS => self.obp0 = value & !0b11,
            OBP1_ADDRESS => self.obp1 = value & !0b11,
            _ => return None,
        }

        Some(())
    }
}
