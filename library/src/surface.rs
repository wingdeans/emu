pub const SCREEN_WIDTH: u32 = 160;
pub const SCREEN_HEIGHT: u32 = 144;

#[derive(PartialEq, Clone, Copy)]
pub enum Layer {
    None,
    Background,
    Window,
    Object,
}

pub struct Metadata {
    pub layer: Layer,
    pub tile: u8,
}

pub trait Surface {
    fn set_pixel(&mut self, x: u32, y: u32, r: u8, g: u8, b: u8, metadata: Metadata);
    fn flush(&mut self);
}
