use crate::Graphics;
use eframe::egui::{
    Color32, ColorImage, Context, Image, ImageData, Slider, TextureHandle, TextureOptions, Ui,
};
use library::surface::{Layer, Metadata, Surface, SCREEN_HEIGHT, SCREEN_WIDTH};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

pub struct Display {
    image: ColorImage,
    texture: Option<TextureHandle>,
    scale: f32,
    flush: bool,
    graphics: Rc<RefCell<Graphics>>,
}

impl Display {
    pub fn new(graphics: Rc<RefCell<Graphics>>) -> Self {
        Self {
            image: ColorImage::new(
                [SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize],
                Color32::BLACK,
            ),
            texture: None,
            scale: 2.0,
            flush: false,
            graphics,
        }
    }

    pub fn draw(&mut self, ctx: &Context, ui: &mut Ui) {
        if let Some(texture) = &mut self.texture {
            if self.flush {
                texture.set(
                    ImageData::Color(Arc::new(self.image.clone())),
                    Default::default(),
                );
            }

            self.flush = false;
        } else {
            self.texture =
                Some(ctx.load_texture("display", self.image.clone(), TextureOptions::default()));
        }

        ui.add(
            Slider::new(&mut self.scale, 1.0..=5.0)
                .show_value(false)
                .step_by(1.0),
        );

        if let Some(texture) = &self.texture {
            ui.add(Image::new(texture).fit_to_original_size(self.scale));
        }
    }
}

impl Surface for Display {
    fn set_pixel(&mut self, x: u32, y: u32, r: u8, g: u8, b: u8, metadata: Metadata) {
        let graphics = self.graphics.borrow();

        if match metadata.layer {
            Layer::None => true,
            Layer::Background => graphics.show_background,
            Layer::Window => graphics.show_window,
            Layer::Object => graphics.show_objects,
        } {
            if (0..SCREEN_WIDTH).contains(&x) && (0..SCREEN_HEIGHT).contains(&y) {
                self.image.pixels[(y * SCREEN_WIDTH + x) as usize] =
                    if graphics.show_tiles && metadata.layer != Layer::None {
                        Color32::from_gray(metadata.tile)
                    } else {
                        Color32::from_rgb(r, g, b)
                    };
            }
        }
    }

    fn flush(&mut self) {
        self.flush = true;
    }
}
