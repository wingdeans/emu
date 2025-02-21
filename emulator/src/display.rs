use eframe::egui::{
    Color32, ColorImage, Context, Image, ImageData, Slider, TextureHandle, TextureOptions, Ui,
};
use library::surface::{Surface, SCREEN_HEIGHT, SCREEN_WIDTH};
use std::sync::Arc;

pub struct Display {
    image: ColorImage,
    texture: Option<TextureHandle>,
    scale: f32,
}

impl Display {
    pub fn draw(&mut self, ctx: &Context, ui: &mut Ui) {
        if let Some(texture) = &mut self.texture {
            texture.set(
                ImageData::Color(Arc::new(self.image.clone())),
                Default::default(),
            );
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

impl Default for Display {
    fn default() -> Display {
        Self {
            image: ColorImage::new(
                [SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize],
                Color32::BLACK,
            ),
            texture: None,
            scale: 2.0,
        }
    }
}

impl Surface for Display {
    fn set_pixel(&mut self, x: u32, y: u32, r: u8, g: u8, b: u8) {
        if (0..SCREEN_WIDTH).contains(&x) && (0..SCREEN_HEIGHT).contains(&y) {
            self.image.pixels[(y * SCREEN_WIDTH + x) as usize] = Color32::from_rgb(r, g, b);
        }
    }
}
