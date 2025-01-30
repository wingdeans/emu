mod cpu;
mod error;

use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 720.0]),
        ..Default::default()
    };

    eframe::run_native("Emulator", options, Box::new(|_| Ok(Box::new(App::new()))))
}

struct App {}

impl App {
    pub fn new() -> Self {
        Self {}
    }
}

impl eframe::App for App {
    fn update(&mut self, _: &egui::Context, _frame: &mut eframe::Frame) {}
}
