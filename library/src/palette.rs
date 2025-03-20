use crate::bus::Addressable;

pub struct Color(pub u8, pub u8, pub u8);

impl Color {
    pub fn from_monochrome(value: u8) -> Self {
        match value % 4 {
            0 => Self(0xff, 0xff, 0xff),
            1 => Self(0xa9, 0xa9, 0xa9),
            2 => Self(0x54, 0x54, 0x54),
            3 => Self(0x00, 0x00, 0x00),
            _ => unreachable!(),
        }
    }

    pub fn from_555(value: u16) -> Self {
        let r = ((value & 0b11111) as f32) / 63.0 * 255.0;
        let g = (((value >> 5) & 0b11111) as f32) / 63.0 * 255.0;
        let b = (((value >> 10) & 0b11111) as f32) / 63.0 * 255.0;

        Color(r as u8, g as u8, b as u8)
    }
}

pub const BGP_ADDRESS: u16 = 0xff47;
pub const OBP0_ADDRESS: u16 = 0xff48;
pub const OBP1_ADDRESS: u16 = 0xff49;
pub const BCPS_BGPI_ADDRESS: u16 = 0xff68;
pub const BCPD_BGPD_ADDRESS: u16 = 0xff69;
pub const OCPS_OBPI_ADDRESS: u16 = 0xff6a;
pub const OCPD_OBPD_ADDRESS: u16 = 0xff6b;

struct CgbPalette {
    memory: [u8; 64],
    index: usize,
    increment: bool,
}

impl CgbPalette {
    pub fn index(&mut self, value: u8) {
        self.index = (value & 0b11111) as usize;
        self.increment = (value & (1 << 7)) != 0;
    }

    pub fn write(&mut self, value: u8) {
        self.memory[self.index] = value;

        if self.increment {
            self.index = (self.index + 1) % self.memory.len();
        }
    }

    pub fn read(&self) -> u8 {
        self.memory[self.index]
    }

    pub fn color(&self, index: u8) -> Color {
        let addr = ((index as usize) & !1) % self.memory.len();
        let value = u16::from_le_bytes([self.memory[addr], self.memory[addr + 1]]);
        Color::from_555(value)
    }
}

impl Default for CgbPalette {
    fn default() -> Self {
        Self {
            memory: [0; 64],
            index: 0,
            increment: false,
        }
    }
}

pub struct Palette {
    bgp: u8,
    obp0: u8,
    obp1: u8,
    bg: CgbPalette,
    obj: CgbPalette,
    compat_mode: bool,
}

impl Palette {
    pub fn new(compat_mode: bool) -> Self {
        Self {
            bgp: 0,
            obp0: 0,
            obp1: 0,
            bg: Default::default(),
            obj: Default::default(),
            compat_mode,
        }
    }

    pub fn get_bgp(&self, id: u8) -> Color {
        Color::from_monochrome(self.bgp >> ((id % 4) * 2))
    }

    pub fn get_obp0(&self, id: u8) -> Color {
        Color::from_monochrome(self.obp0 >> ((id % 4) * 2))
    }

    pub fn get_obp1(&self, id: u8) -> Color {
        Color::from_monochrome(self.obp1 >> ((id % 4) * 2))
    }

    pub fn get_bg(&self, palette: u8, color: u8) -> Color {
        if self.compat_mode && palette == 0 {
            self.get_bgp(color)
        } else {
            self.bg.color(palette * 8 + color * 2)
        }
    }

    pub fn get_obj(&self, palette: u8, color: u8) -> Color {
        if self.compat_mode && palette == 0 {
            self.get_obp0(color)
        } else if self.compat_mode {
            self.get_obp1(color)
        } else {
            self.obj.color(palette * 8 + color * 2)
        }
    }
}

impl Addressable for Palette {
    fn read(&mut self, addr: u16) -> Option<u8> {
        match addr {
            BGP_ADDRESS => Some(self.bgp),
            OBP0_ADDRESS => Some(self.obp0),
            OBP1_ADDRESS => Some(self.obp1),
            BCPD_BGPD_ADDRESS => Some(self.bg.read()),
            OCPD_OBPD_ADDRESS => Some(self.obj.read()),
            _ => None,
        }
    }

    fn write(&mut self, addr: u16, value: u8) -> Option<()> {
        match addr {
            BGP_ADDRESS => self.bgp = value,
            OBP0_ADDRESS => self.obp0 = value,
            OBP1_ADDRESS => self.obp1 = value,
            BCPS_BGPI_ADDRESS => self.bg.index(value),
            BCPD_BGPD_ADDRESS => self.bg.write(value),
            OCPS_OBPI_ADDRESS => self.obj.index(value),
            OCPD_OBPD_ADDRESS => self.obj.write(value),
            _ => return None,
        }

        Some(())
    }
}
