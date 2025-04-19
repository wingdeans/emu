use crate::{
    bus::Addressable,
    dma::{Dma, HDMA1_ADDRESS, HDMA5_ADDRESS, OAM_DMA_ADDRESS},
    int::{Interrupt, STAT_INT_FLAG, VBLANK_INT_FLAG},
    palette::Palette,
    surface::{Surface, SCREEN_WIDTH},
    system::OAM_BEGIN,
};
use std::{cell::RefCell, rc::Rc};

const VBLANK_HEIGHT_BEGIN: u32 = 144;
const MAX_SCANLINE_HEIGHT: u32 = 153;

pub const LCD_Y_ADDRESS: u16 = 0xff44;
pub const LYC_ADDRESS: u16 = 0xff45;
pub const SCX_ADDRESS: u16 = 0xff43;
pub const SCY_ADDRESS: u16 = 0xff42;
pub const WX_ADDRESS: u16 = 0xff4a;
pub const WY_ADDRESS: u16 = 0xff4b;
pub const LCDC_ADDRESS: u16 = 0xff40;
pub const STAT_ADDRESS: u16 = 0xff41;
pub const BLOCK0_ADDRESS: u16 = 0x8000;
pub const BLOCK1_ADDRESS: u16 = 0x8800;
pub const BLOCK2_ADDRESS: u16 = 0x9000;

pub const TILE_BYTES: usize = 16;
pub const TILE_SIZE: u32 = 8;
pub const BG_SIZE: u32 = 32;

pub struct Ppu {
    surface: Rc<RefCell<dyn Surface>>,
    vram0: Rc<RefCell<dyn Addressable>>,
    vram1: Rc<RefCell<dyn Addressable>>,
    oam: Rc<RefCell<dyn Addressable>>,
    palette: Rc<RefCell<Palette>>,
    int: Rc<RefCell<Interrupt>>,
    dma: Rc<RefCell<Dma>>,
    render_y: u8,
    bg_x: u8,
    bg_y: u8,
    wnd_x: u8,
    wnd_y: u8,
    lcdc: u8,
    stat: u8,
    lyc: u8,
}

impl Ppu {
    pub fn new(
        surface: Rc<RefCell<dyn Surface>>,
        bus: Rc<RefCell<dyn Addressable>>,
        vram0: Rc<RefCell<dyn Addressable>>,
        vram1: Rc<RefCell<dyn Addressable>>,
        oam: Rc<RefCell<dyn Addressable>>,
        palette: Rc<RefCell<Palette>>,
        int: Rc<RefCell<Interrupt>>,
    ) -> Self {
        Self {
            surface,
            vram0,
            vram1,
            oam,
            palette,
            int,
            dma: Rc::new(RefCell::new(Dma::new(bus))),
            render_y: 0,
            bg_x: 0,
            bg_y: 0,
            wnd_x: 0,
            wnd_y: 0,
            lcdc: 0x91,
            stat: 0,
            lyc: 0,
        }
    }

    fn blit_tile(
        &self,
        surface: &mut dyn Surface,
        vram: &mut dyn Addressable,
        palette: &Palette,
        idx: u8,
        obj: bool,
        x: i16,
        y: i16,
        attr: u8,
    ) {
        if obj && (self.lcdc & 1) == 0 || y > self.render_y as i16 || y <= -(TILE_SIZE as i16) {
            return;
        }

        let base = if idx >= 128 {
            BLOCK1_ADDRESS - (128 * TILE_BYTES as u16)
        } else if obj || self.lcdc & 0x10 != 0 {
            BLOCK0_ADDRESS
        } else {
            BLOCK2_ADDRESS
        };

        let addr = base
            + (TILE_BYTES as u16) * (idx as u16)
            + ((self.render_y as i16 - y) as u16) * (TILE_BYTES / TILE_SIZE as usize) as u16;

        let lo = vram.read(addr).unwrap();
        let hi = vram.read(addr + 1).unwrap();

        for i in 0..TILE_SIZE as u8 {
            if obj && ((x + i as i16) < 0 || (x + i as i16) >= SCREEN_WIDTH as i16)
                || (x + i as i16) < 0
                || (x + i as i16) >= SCREEN_WIDTH as i16
            {
                continue;
            }

            let col = (if lo & (1 << (7 - i)) != 0 { 1 } else { 0 })
                | (if (hi & (1 << (7 - i))) != 0 { 2 } else { 0 });

            if obj && col == 0 {
                continue;
            }

            let color = if obj && attr & 16 != 0 {
                palette.get_obp1(col)
            } else if obj {
                palette.get_obp0(col)
            } else {
                palette.get_bg(0, col)
            };

            surface.set_pixel(
                (x + i as i16) as u32,
                self.render_y as u32,
                color.0,
                color.1,
                color.2,
            );
        }
    }

    fn draw_obj(
        &self,
        surface: &mut dyn Surface,
        vram: &mut dyn Addressable,
        oam: &mut dyn Addressable,
        palette: &Palette,
    ) {
        let mut objects = 0;
        let tall = self.lcdc & 4 != 0;

        for i in 0..40 {
            let addr = OAM_BEGIN + i * 4;
            let y = oam.read(addr).unwrap() as i16 - 16;

            if (self.render_y as i16) < y
                || (!tall && self.render_y as i16 >= y + TILE_SIZE as i16)
                || (tall && self.render_y as i16 >= y + (TILE_SIZE * 2) as i16)
                || objects == 10
            {
                continue;
            }

            objects += 1;

            let x = oam.read(addr + 1).unwrap() as i16 - 8;
            let idx = oam.read(addr + 2).unwrap();
            let attr = oam.read(addr + 3).unwrap();

            let tile = if tall && self.render_y as i16 >= y + TILE_SIZE as i16 {
                (idx & 0xfe) + 1
            } else if tall {
                idx & 0xfe
            } else {
                idx
            };

            self.blit_tile(surface, vram, palette, tile, true, x, y as i16, attr);
        }
    }

    fn draw_bg(
        &self,
        surface: &mut dyn Surface,
        vram0: &mut dyn Addressable,
        _vram1: &mut dyn Addressable,
        palette: &Palette,
    ) {
        let y = self.render_y.wrapping_add(self.bg_y);
        let tile_y = y / TILE_SIZE as u8;

        for tile_x in 0..(SCREEN_WIDTH / TILE_SIZE) + 1 {
            let bx = ((tile_x * TILE_SIZE) as u8).wrapping_add(self.bg_x) / TILE_SIZE as u8;

            let addr = 0x9800
                | (((self.lcdc & (1 << 3)) as u16) << 7)
                | ((tile_y as u16) << 5)
                | (bx as u16);
            let tile = vram0.read(addr).unwrap();

            self.blit_tile(
                surface,
                vram0,
                palette,
                tile,
                false,
                (tile_x * TILE_SIZE) as i16 - (self.bg_x % TILE_SIZE as u8) as i16,
                (self.render_y - (self.render_y % TILE_SIZE as u8)) as i16
                    - (self.bg_y % TILE_SIZE as u8) as i16,
                0,
            );
        }
    }

    pub fn scanline(&mut self) {
        if self.lcdc & 0x80 == 0 {
            return;
        }

        if self.render_y < VBLANK_HEIGHT_BEGIN as u8 {
            if self.stat & 0x20 != 0 {
                self.int.borrow_mut().int(STAT_INT_FLAG);
            }

            let mut surface = self.surface.borrow_mut();
            let mut vram0 = self.vram0.borrow_mut();
            let mut vram1 = self.vram1.borrow_mut();
            let mut oam = self.oam.borrow_mut();
            let palette = self.palette.borrow();

            self.draw_bg(&mut *surface, &mut *vram0, &mut *vram1, &*palette);

            if self.lcdc & 2 != 0 {
                self.draw_obj(&mut *surface, &mut *vram0, &mut *oam, &*palette);
            }

            if self.stat & 0x08 != 0 {
                self.int.borrow_mut().int(STAT_INT_FLAG);
            }
        } else if self.render_y == VBLANK_HEIGHT_BEGIN as u8 {
            self.int.borrow_mut().int(VBLANK_INT_FLAG);

            if self.stat & 0x10 != 0 {
                self.int.borrow_mut().int(STAT_INT_FLAG);
            }

            self.surface.borrow_mut().flush();
        }

        self.render_y = (self.render_y + 1) % (MAX_SCANLINE_HEIGHT as u8 + 1);

        if self.lyc == self.render_y && self.stat & 0x40 != 0 {
            self.int.borrow_mut().int(STAT_INT_FLAG);
        }
    }

    pub fn get_lcdc(&self) -> u8 {
        self.lcdc
    }

    pub fn ref_dma(&self) -> &Rc<RefCell<Dma>> {
        &self.dma
    }
}

impl Addressable for Ppu {
    fn read(&mut self, addr: u16) -> Option<u8> {
        match addr {
            LCD_Y_ADDRESS => Some(self.render_y),
            LYC_ADDRESS => Some(self.lyc),
            SCX_ADDRESS => Some(self.bg_x),
            SCY_ADDRESS => Some(self.bg_y),
            WX_ADDRESS => Some(self.wnd_x),
            WY_ADDRESS => Some(self.wnd_y),
            LCDC_ADDRESS => Some(self.lcdc),
            STAT_ADDRESS => Some(
                (self.stat & !0b111)
                    | (self.lcdc >> 7)
                    | (if self.render_y == self.lyc { 4 } else { 0 }),
            ),
            HDMA1_ADDRESS..HDMA5_ADDRESS | OAM_DMA_ADDRESS => self.dma.borrow_mut().read(addr),
            _ => None,
        }
    }

    fn write(&mut self, addr: u16, value: u8) -> Option<()> {
        match addr {
            LYC_ADDRESS => self.lyc = value,
            SCX_ADDRESS => self.bg_x = value,
            SCY_ADDRESS => self.bg_y = value,
            WX_ADDRESS => self.wnd_x = value,
            WY_ADDRESS => self.wnd_y = value,
            LCDC_ADDRESS => self.lcdc = value,
            STAT_ADDRESS => self.stat = value,
            HDMA1_ADDRESS..HDMA5_ADDRESS | OAM_DMA_ADDRESS => {
                return self.dma.borrow_mut().write(addr, value)
            }
            _ => return None,
        }

        Some(())
    }
}
