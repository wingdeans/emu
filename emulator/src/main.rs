mod display;

use crate::display::Display;
use cpu::cpu::{Cpu, Result as CpuResult};
use eframe::{
    egui::{self, Button, Grid, Label, Layout, RichText, Window},
    epaint::Color32,
};
use egui_extras::{Column, TableBuilder};
use egui_memory_editor::MemoryEditor;
use library::{
    bus::Addressable, cartridge::Cartridge, clock::Clock, input::InputHandler, palette::Color,
    surface::Surface, system::System,
};
use std::{cell::RefCell, env, path::Path, rc::Rc};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 720.0]),
        ..Default::default()
    };

    eframe::run_native("Emulator", options, Box::new(|_| Ok(Box::new(App::new()))))
}

#[derive(Default)]
struct EguiInputHandler {
    pub start: bool,
    pub select: bool,
    pub b: bool,
    pub a: bool,
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

impl InputHandler for EguiInputHandler {
    fn start(&self) -> bool {
        self.start
    }

    fn select(&self) -> bool {
        self.select
    }

    fn b(&self) -> bool {
        self.b
    }

    fn a(&self) -> bool {
        self.a
    }

    fn up(&self) -> bool {
        self.up
    }

    fn down(&self) -> bool {
        self.down
    }

    fn left(&self) -> bool {
        self.left
    }

    fn right(&self) -> bool {
        self.right
    }
}

struct State {
    control_open: bool,
    memory_editor_open: bool,
    registers_open: bool,
    display_open: bool,
    palette_open: bool,
    ppu_open: bool,
}

struct App {
    cpu: Cpu,
    memory_editor: MemoryEditor,
    state: State,
    cartridge: Rc<RefCell<Cartridge>>,
    last_execute: Option<CpuResult<u32>>,
    system: Rc<RefCell<System>>,
    display: Rc<RefCell<Display>>,
    clock: Rc<RefCell<Clock>>,
    running: bool,
    input: Rc<RefCell<EguiInputHandler>>,
}

impl App {
    pub fn new() -> Self {
        let args: Vec<_> = env::args().collect();
        let path = &args[1];

        let display = Rc::new(RefCell::new(Display::default()));

        let cartridge = Rc::new(RefCell::new(
            Cartridge::load_from_file(Path::new(path)).expect("Failed to load cartridge"),
        ));

        let input = Rc::new(RefCell::new(EguiInputHandler::default()));

        let system = Rc::new(RefCell::new(System::new(
            Rc::clone(&cartridge),
            Rc::clone(&display) as Rc<RefCell<dyn Surface>>,
            Rc::clone(&input) as Rc<RefCell<dyn InputHandler>>,
        )));

        let clock = Rc::new(RefCell::new(Clock::new(Rc::clone(
            system.borrow().ppu_ref(),
        ))));

        Self {
            cpu: Cpu::new(Rc::clone(&system) as Rc<RefCell<dyn Addressable>>),
            memory_editor: MemoryEditor::new().with_address_range("All", 0..0xffff),
            state: State {
                control_open: true,
                memory_editor_open: true,
                registers_open: true,
                display_open: true,
                palette_open: true,
                ppu_open: true,
            },
            cartridge,
            last_execute: None,
            system,
            display,
            clock,
            running: false,
            input,
        }
    }
}

impl App {
    fn draw_control(&mut self, ctx: &egui::Context) {
        use cpu::cpu::State::*;

        let color = if self.running {
            Color32::from_rgb(0, 0, 255)
        } else {
            match self.cpu.state() {
                Stopped => Color32::from_rgb(255, 0, 0),
                Halted => Color32::from_rgb(255, 255, 0),
                Running => Color32::from_rgb(0, 255, 0),
            }
        };

        Window::new("Control")
            .open(&mut self.state.control_open)
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.with_layout(Layout::left_to_right(egui::Align::LEFT), |ui| {
                    ui.set_width(300.0);

                    ui.add_sized([25.0, 25.0], Label::new(RichText::new("⏺").color(color)));

                    if !self.running && ui.add_sized([25.0, 25.0], Button::new("▶")).clicked() {
                        self.running = true;
                    }

                    if self.running && ui.add_sized([25.0, 25.0], Button::new("⏹")).clicked() {
                        self.running = false;
                    }

                    if ui.add_sized([25.0, 25.0], Button::new("↺")).clicked() {
                        self.system = Rc::new(RefCell::new(System::new(
                            Rc::clone(&self.cartridge),
                            Rc::clone(&self.display) as Rc<RefCell<dyn Surface>>,
                            Rc::new(RefCell::new(EguiInputHandler::default()))
                                as Rc<RefCell<dyn InputHandler>>,
                        )));
                        self.cpu =
                            Cpu::new(Rc::clone(&self.system) as Rc<RefCell<dyn Addressable>>);
                        self.last_execute = None;
                    }

                    if let Some(exe) = &self.last_execute {
                        match exe {
                            Ok(cycles) => {
                                ui.label(RichText::new(format!("Ok: {}", cycles)).size(18.0))
                            }
                            Err(e) => ui.label(RichText::new(format!("Err: {}", e)).size(18.0)),
                        };
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

    fn draw_palette(&mut self, ctx: &egui::Context) {
        Window::new("Palette")
            .open(&mut self.state.palette_open)
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                let swatch = |ui: &mut egui::Ui, name: &str, col: Color| {
                    let (rect, response) =
                        ui.allocate_exact_size(egui::vec2(40.0, 40.0), egui::Sense::hover());

                    ui.painter()
                        .rect_filled(rect, 0.0, Color32::from_rgb(col.0, col.1, col.2));

                    response.on_hover_text(name);
                };

                let system = self.system.borrow();
                let palette = system.palette_ref().borrow();

                ui.heading("Monochrome");
                Grid::new("monochrome").spacing([1.0, 1.0]).show(ui, |ui| {
                    for i in 0..4 {
                        swatch(ui, &format!("BGP {}", 3 - i), palette.get_bgp(3 - i));
                    }
                    ui.end_row();

                    for i in 1..4 {
                        swatch(ui, &format!("OBP0 {}", 3 - i), palette.get_obp0(3 - i));
                    }
                    ui.end_row();

                    for i in 1..4 {
                        swatch(ui, &format!("OBP1 {}", 3 - i), palette.get_obp1(3 - i));
                    }
                    ui.end_row();
                });

                ui.heading("Color");
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        Grid::new("color_bg").spacing([1.0, 1.0]).show(ui, |ui| {
                            for p in 0..8 {
                                for c in 0..4 {
                                    swatch(
                                        ui,
                                        &format!("BG{} {}", p, 3 - c),
                                        palette.get_bg(p, 3 - c),
                                    );
                                }

                                ui.end_row();
                            }
                        });
                    });

                    ui.vertical(|ui| {
                        Grid::new("color_obp").spacing([1.0, 1.0]).show(ui, |ui| {
                            for p in 0..8 {
                                for c in 1..4 {
                                    swatch(
                                        ui,
                                        &format!("OBP{} {}", p, 3 - c),
                                        palette.get_obj(p, 3 - c),
                                    );
                                }

                                ui.end_row();
                            }
                        });
                    });
                });
            });
    }

    fn draw_ppu(&mut self, ctx: &egui::Context) {
        Window::new("PPU")
            .open(&mut self.state.ppu_open)
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                let lcdc = self.system.borrow().ppu_ref().borrow().get_lcdc();

                ui.ctx().input(|input| {
                    let mut handler = self.input.borrow_mut();
                    handler.start = input.key_down(egui::Key::S);
                    handler.select = input.key_down(egui::Key::A);
                    handler.b = input.key_down(egui::Key::Z);
                    handler.a = input.key_down(egui::Key::X);
                    handler.up = input.key_down(egui::Key::ArrowUp);
                    handler.down = input.key_down(egui::Key::ArrowDown);
                    handler.left = input.key_down(egui::Key::ArrowLeft);
                    handler.right = input.key_down(egui::Key::ArrowRight);
                });

                TableBuilder::new(ui)
                    .column(Column::exact(100.0))
                    .column(Column::exact(100.0))
                    .body(|mut body| {
                        let mut add = |label: &str, bit: u8| {
                            body.row(20.0, |mut row| {
                                row.col(|ui| _ = ui.label(label));
                                row.col(|ui| {
                                    _ = ui.add_sized(
                                        [20.0, 20.0],
                                        Label::new(RichText::new("⏺").color(if lcdc & bit != 0 {
                                            Color32::from_rgb(0, 255, 0)
                                        } else {
                                            Color32::from_rgb(255, 0, 0)
                                        })),
                                    )
                                });
                            });
                        };

                        add("LCD", 0x80);
                        add("OBJ", 0x01);

                        body.row(20.0, |mut row| {
                            row.col(|ui| _ = ui.label("BG"));
                            row.col(|ui| {
                                ui.monospace(if lcdc & (1 << 3) != 0 {
                                    "0x9c00"
                                } else {
                                    "0x9800"
                                });
                            });
                        });
                    });
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
                ui.toggle_value(&mut self.state.palette_open, "Palette");
                ui.toggle_value(&mut self.state.ppu_open, "PPU");
            });
        });

        self.draw_control(ctx);
        self.draw_memory_editor(ctx);
        self.draw_registers(ctx);
        self.draw_display(ctx);
        self.draw_palette(ctx);
        self.draw_ppu(ctx);

        let mut cont = true;
        while self.running && cont {
            self.last_execute = Some(self.cpu.execute());
            if let Some(x) = &self.last_execute {
                match x {
                    Ok(y) => cont = !self.clock.borrow_mut().clock(*y),
                    Err(_) => self.running = false,
                }
            }
        }

        if self.running {
            ctx.request_repaint();
        }
    }
}
