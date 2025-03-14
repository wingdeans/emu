use crate::{
    bus::Addressable,
    dma::Dma,
    palette::Palette,
    surface::{Surface, SCREEN_WIDTH},
};
use std::{cell::RefCell, rc::Rc};

const VBLANK_HEIGHT_BEGIN: u32 = 144;
const MAX_SCANLINE_HEIGHT: u32 = 153;

pub const LCD_Y_ADDRESS: u16 = 0xff44;
pub const SCX_ADDRESS: u16 = 0xff43;
pub const SCY_ADDRESS: u16 = 0xff42;
pub const WX_ADDRESS: u16 = 0xff4a;
pub const WY_ADDRESS: u16 = 0xff4b;
pub const LCDC_ADDRESS: u16 = 0xff40;

enum Map {
    Low,
    High,
}

impl Map {
    pub fn from(lcdc: u8) -> Self {
        if lcdc & (1 << 3) == 0 {
            Map::Low
        } else {
            Map::High
        }
    }

    pub fn address(&self) -> u16 {
        use Map::*;

        match self {
            Low => 0x9800,
            High => 0x9c00,
        }
    }
}

struct TileMap<'a> {
    vram: &'a mut dyn Addressable,
    base: u16,
}

impl<'a> TileMap<'a> {
    pub fn from(vram: &'a mut dyn Addressable, map: &Map) -> Self {
        Self {
            vram,
            base: map.address(),
        }
    }
}

pub struct Ppu {
    surface: Rc<RefCell<dyn Surface>>,
    vram: Rc<RefCell<dyn Addressable>>,
    palette: Rc<RefCell<Palette>>,
    dma: Dma,
    render_y: u8,
    bg_x: u8,
    bg_y: u8,
    wnd_x: u8,
    wnd_y: u8,
    lcdc: u8,
}

impl Ppu {
    pub fn new(
        surface: Rc<RefCell<dyn Surface>>,
        bus: Rc<RefCell<dyn Addressable>>,
        vram: Rc<RefCell<dyn Addressable>>,
        palette: Rc<RefCell<Palette>>,
    ) -> Self {
        Self {
            surface,
            vram,
            palette,
            dma: Dma::new(bus),
            render_y: 0,
            bg_x: 0,
            bg_y: 0,
            wnd_x: 0,
            wnd_y: 0,
            lcdc: 0,
        }
    }

    pub fn scanline(&mut self) {
        if self.render_y < VBLANK_HEIGHT_BEGIN as u8 {
            let mut surface = self.surface.borrow_mut();
            let mut vram = self.vram.borrow_mut();

            let map = TileMap::from(&mut *vram, &Map::from(self.lcdc));

            for x in 0..SCREEN_WIDTH {
                surface.set_pixel(x, self.render_y as u32, 255, 0, 0);
            }

            surface.flush();
            self.dma.scanline();
        }

        self.render_y = (self.render_y + 1) % (MAX_SCANLINE_HEIGHT as u8 + 1);
    }

    pub fn get_render_y(&self) -> u8 {
        self.render_y
    }
}

impl Addressable for Ppu {
    fn read(&mut self, addr: u16) -> Option<u8> {
        match addr {
            LCD_Y_ADDRESS => Some(self.render_y),
            SCX_ADDRESS => Some(self.bg_x),
            SCY_ADDRESS => Some(self.bg_y),
            WX_ADDRESS => Some(self.wnd_x),
            WY_ADDRESS => Some(self.wnd_y),
            LCDC_ADDRESS => Some(self.lcdc),
            _ => self.dma.read(addr),
        }
    }

    fn write(&mut self, addr: u16, value: u8) -> Option<()> {
        match addr {
            SCX_ADDRESS => self.bg_x = value,
            SCY_ADDRESS => self.bg_y = value,
            WX_ADDRESS => self.wnd_x = value,
            WY_ADDRESS => self.wnd_y = value,
            LCDC_ADDRESS => self.lcdc = value,
            _ => return self.dma.write(addr, value),
        }

        Some(())
    }
}
