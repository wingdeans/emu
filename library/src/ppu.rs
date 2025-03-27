use crate::{
    bus::Addressable,
    dma::Dma,
    int::{Interrupt, STAT_INT_FLAG, VBLANK_INT_FLAG},
    palette::Palette,
    surface::{Surface, SCREEN_HEIGHT, SCREEN_WIDTH},
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
    dma: Dma,
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
            dma: Dma::new(bus),
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
        y: u8,
        attr: u8,
    ) {
        if obj && (self.lcdc & 1) == 0 {
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
            + ((self.render_y - y) as u16) * (TILE_BYTES / TILE_SIZE as usize) as u16;

        let lo = vram.read(addr).unwrap();
        let hi = vram.read(addr + 1).unwrap();

        for i in 0..TILE_SIZE as u8 {
            if obj && ((x + i as i16) < 0 || (x + i as i16) >= SCREEN_WIDTH as i16) {
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
                (x as u8 + i) as u32 % SCREEN_WIDTH,
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

            self.blit_tile(surface, vram, palette, tile, true, x, y as u8, attr);
        }
    }

    fn draw_bg(
        &self,
        surface: &mut dyn Surface,
        vram0: &mut dyn Addressable,
        vram1: &mut dyn Addressable,
        palette: &Palette,
    ) {
        let top = self.bg_y;
        let bottom = self.bg_y.wrapping_add(SCREEN_HEIGHT as u8);

        if self.render_y < top || self.render_y >= bottom {
            return;
        }

        let y = self.render_y.wrapping_sub(top);
        let tile_y = y / TILE_SIZE as u8;

        for tile_x in 0..(SCREEN_WIDTH / TILE_SIZE) {
            let addr = 0x9800
                | (((self.lcdc & (1 << 3)) as u16) << 7)
                | ((tile_y as u16) << 5)
                | (tile_x as u16);
            let tile = vram0.read(addr).unwrap();

            let x =
                ((tile_x as u32 * TILE_SIZE + SCREEN_WIDTH * 2) - self.bg_x as u32) % SCREEN_WIDTH;
            self.blit_tile(
                surface,
                vram0,
                palette,
                tile,
                false,
                x as i16,
                tile_y * TILE_SIZE as u8,
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

            self.dma.scanline();
            self.dma.oam();
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
            _ => self.dma.read(addr),
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
            _ => return self.dma.write(addr, value),
        }

        Some(())
    }
}
