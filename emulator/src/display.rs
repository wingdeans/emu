use eframe::egui::{Color32, ColorImage, Context, ImageData, TextureHandle};
use library::surface::{self, Surface};
use std::sync::Arc;

pub struct Display {
    image: ColorImage,
    texture: Option<TextureHandle>,
}

impl Display {
    pub fn draw(&mut self, ctx: &Context) {
        if self.texture.is_none() {}

        texture.set(
            ImageData::Color(Arc::new(self.image.clone())),
            Default::default(),
        );
    }
}

impl Default for Display {
    fn default() -> Display {
        Self {
            image: ColorImage::new(
                [
                    surface::SCREEN_WIDTH as usize,
                    surface::SCREEN_HEIGHT as usize,
                ],
                Color32::BLACK,
            ),
            texture: None,
        }
    }
}

impl Surface for Display {
    type Error = &'static str;

    fn set_pixel(&mut self, x: u32, y: u32, r: u8, g: u8, b: u8) -> Result<(), Self::Error> {
        if !Self::is_valid(x, y) {
            return Err("Out of bounds");
        }

        self.image.pixels[(y * surface::SCREEN_WIDTH + x) as usize] = Color32::from_rgb(r, g, b);
        Ok(())
    }
}
