mod display;

use crate::display::Display;
use cpu::cpu::{Cpu, Result as CpuResult};
use eframe::{
    egui::{self, Button, Label, Layout, RichText, Window},
    epaint::Color32,
};
use egui_extras::{Column, TableBuilder};
use egui_memory_editor::MemoryEditor;
use library::{bus::Addressable, cartridge::Cartridge, system::System};
use std::{cell::RefCell, env, path::Path, rc::Rc};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 720.0]),
        ..Default::default()
    };

    eframe::run_native("Emulator", options, Box::new(|_| Ok(Box::new(App::new()))))
}

struct State {
    control_open: bool,
    memory_editor_open: bool,
    registers_open: bool,
    display_open: bool,
}

struct App {
    cpu: Cpu,
    memory_editor: MemoryEditor,
    state: State,
    last_execute: Option<CpuResult<u32>>,
    system: Rc<RefCell<System>>,
    display: Rc<RefCell<Display>>,
}

impl App {
    pub fn new() -> Self {
        let args: Vec<_> = env::args().collect();
        let path = &args[1];

        let cartridge =
            Cartridge::load_from_file(Path::new(path)).expect("Failed to load cartridge");
        let system = Rc::new(RefCell::new(System::new(cartridge)));

        Self {
            cpu: Cpu::new(Rc::clone(&system) as Rc<RefCell<dyn Addressable>>),
            memory_editor: MemoryEditor::new().with_address_range("All", 0..0xffff),
            state: State {
                control_open: true,
                memory_editor_open: true,
                registers_open: true,
                display_open: true,
            },
            last_execute: None,
            system,
            display: Default::default(),
        }
    }
}

impl App {
    fn draw_control(&mut self, ctx: &egui::Context) {
        use cpu::cpu::State::*;

        let color = match self.cpu.state() {
            Stopped => Color32::from_rgb(255, 0, 0),
            Halted => Color32::from_rgb(255, 255, 0),
            Running => Color32::from_rgb(0, 255, 0),
        };

        Window::new("Control")
            .open(&mut self.state.control_open)
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.with_layout(Layout::left_to_right(egui::Align::LEFT), |ui| {
                    ui.set_width(300.0);

                    ui.add_sized([25.0, 25.0], Label::new(RichText::new("⏺").color(color)));

                    if ui.add_sized([25.0, 25.0], Button::new("▶")).clicked() {
                        self.last_execute = Some(self.cpu.execute());
                    }

                    if ui.add_sized([25.0, 25.0], Button::new("↺")).clicked() {
                        self.cpu =
                            Cpu::new(Rc::clone(&self.system) as Rc<RefCell<dyn Addressable>>);
                        self.last_execute = None;
                    }

                    if let Some(exe) = &self.last_execute {
                        ui.label(RichText::new(format!("{:?}", exe)).size(18.0));
                    }
                });
            });
    }

    fn draw_memory_editor(&mut self, ctx: &egui::Context) {
        self.memory_editor.window_ui(
            ctx,
            &mut self.state.memory_editor_open,
            &mut self.system,
            |bus, addr| bus.borrow_mut().read(addr as u16),
            |bus, addr, value| _ = bus.borrow_mut().write(addr as u16, value),
        );
    }

    fn draw_registers(&mut self, ctx: &egui::Context) {
        Window::new("Registers")
            .open(&mut self.state.registers_open)
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                TableBuilder::new(ui)
                    .column(Column::exact(100.0))
                    .column(Column::exact(100.0))
                    .body(|mut body| {
                        let mut add = |label: &str, value: String| {
                            body.row(20.0, |mut row| {
                                row.col(|ui| _ = ui.label(label));
                                row.col(|ui| _ = ui.monospace(value));
                            });
                        };

                        add("A", format!("0x{:02x}", self.cpu.af >> 8));
                        add("F", format!("0b{:08b}", self.cpu.af & 0x0f));
                        add("BC", format!("0x{:04x}", self.cpu.bc));
                        add("DE", format!("0x{:04x}", self.cpu.de));
                        add("HL", format!("0x{:04x}", self.cpu.hl));
                        add("Stack Pointer", format!("0x{:04x}", self.cpu.sp));
                        add("Program Counter", format!("0x{:04x}", self.cpu.pc));
                    });
            });
    }

    fn draw_display(&mut self, ctx: &egui::Context) {
        Window::new("Display")
            .open(&mut self.state.display_open)
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                self.display.borrow_mut().draw(ctx, ui);
            });
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.toggle_value(&mut self.state.control_open, "Control");
                ui.toggle_value(&mut self.state.registers_open, "Registers");
                ui.toggle_value(&mut self.state.memory_editor_open, "Memory Editor");
                ui.toggle_value(&mut self.state.display_open, "Display");
            });
        });

        self.draw_control(ctx);
        self.draw_memory_editor(ctx);
        self.draw_registers(ctx);
        self.draw_display(ctx);
    }
}
