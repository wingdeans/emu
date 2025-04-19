use cpu::cpu::Cpu;
use library::{clock::Clock, surface, system::System};
use std::sync::{Arc, Mutex};
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

struct Driver {
    pub frequency: u32,
    pub generator: Arc<Mutex<dyn library::apu::Generator + Send>>,
}

impl sdl2::audio::AudioCallback for Driver {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        self.generator.lock().unwrap().generate(self.frequency, out);
    }
}

#[derive(Default)]
struct CpuInterruptHandler {
    pub cpu: Weak<RefCell<Cpu>>,
}

impl library::int::InterruptHandler for CpuInterruptHandler {
    fn ime(&self) -> bool {
        self.cpu.upgrade().unwrap().borrow().ime
    }

    fn handle(&mut self, addr: u16) {
        self.cpu.upgrade().unwrap().borrow_mut().int(addr).unwrap();
    }
}

#[derive(Default)]
pub struct Input {
    pub start: bool,
    pub select: bool,
    pub b: bool,
    pub a: bool,
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

impl library::input::InputHandler for Input {
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

pub struct DisplayTexture<'a> {
    texture: &'a RefCell<sdl2::render::Texture<'static>>,
    pixels: [(u8, u8, u8); (surface::SCREEN_WIDTH * surface::SCREEN_HEIGHT) as usize],
}

impl<'a> DisplayTexture<'a> {
    pub fn new(texture: &'a RefCell<sdl2::render::Texture<'static>>) -> Self {
        Self {
            texture,
            pixels: [(0, 0, 0); (surface::SCREEN_WIDTH * surface::SCREEN_HEIGHT) as usize],
        }
    }
}

impl surface::Surface for DisplayTexture<'_> {
    fn set_pixel(&mut self, x: u32, y: u32, r: u8, g: u8, b: u8) {
        self.pixels[(y * surface::SCREEN_WIDTH + x) as usize] = (r, g, b);
    }

    fn flush(&mut self) {
        self.texture
            .borrow_mut()
            .with_lock(None, |buffer, pitch| {
                for i in 0..self.pixels.len() {
                    let pixel = self.pixels[i];
                    let idx = i / (surface::SCREEN_WIDTH as usize) * pitch
                        + i % (surface::SCREEN_WIDTH as usize)
                            * (pitch / surface::SCREEN_WIDTH as usize);

                    buffer[idx + 0] = pixel.0;
                    buffer[idx + 1] = pixel.1;
                    buffer[idx + 2] = pixel.2;
                }
            })
            .unwrap();
    }
}

fn main() -> Result<(), String> {
    let args: Vec<_> = std::env::args().collect();
    let path = std::path::Path::new(&args[1]);

    let ctx = sdl2::init().map_err(|e| e.to_string())?;
    let video = ctx.video().map_err(|e| e.to_string())?;
    let audio = ctx.audio().map_err(|e| e.to_string())?;

    let window = video
        .window("emu", surface::SCREEN_WIDTH * 4, surface::SCREEN_HEIGHT * 4)
        .opengl()
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    let texture = RefCell::new(unsafe {
        std::mem::transmute::<sdl2::render::Texture<'_>, sdl2::render::Texture<'static>>(
            texture_creator
                .create_texture_streaming(
                    sdl2::pixels::PixelFormatEnum::RGB24,
                    surface::SCREEN_WIDTH,
                    surface::SCREEN_HEIGHT,
                )
                .map_err(|e| e.to_string())?,
        )
    });

    {
        let display = Rc::new(RefCell::new(DisplayTexture::new(unsafe {
            std::mem::transmute::<
                &RefCell<sdl2::render::Texture<'static>>,
                &'static RefCell<sdl2::render::Texture<'static>>,
            >(&texture)
        })));
        let input = Rc::new(RefCell::new(Input::default()));
        let int = Rc::new(RefCell::new(CpuInterruptHandler::default()));

        let cartridge = Rc::new(RefCell::new(
            library::cartridge::Cartridge::load_from_file(path).map_err(|e| e.to_string())?,
        ));

        let system = Rc::new(RefCell::new(System::new(
            cartridge,
            display,
            Rc::clone(&input) as Rc<RefCell<dyn library::input::InputHandler>>,
            Rc::clone(&int) as Rc<RefCell<dyn library::int::InterruptHandler>>,
        )));

        let desired_spec = sdl2::audio::AudioSpecDesired {
            freq: Some(44100),
            channels: Some(1),
            samples: None,
        };

        let chn_1 = audio
            .open_playback(None, &desired_spec, |spec| Driver {
                frequency: spec.freq as u32,
                generator: system.borrow().apu_ref().borrow().chn_1.clone(),
            })
            .map_err(|e| e.to_string())?;

        let chn_2 = audio
            .open_playback(None, &desired_spec, |spec| Driver {
                frequency: spec.freq as u32,
                generator: system.borrow().apu_ref().borrow().chn_2.clone(),
            })
            .map_err(|e| e.to_string())?;

        let chn_3 = audio
            .open_playback(None, &desired_spec, |spec| Driver {
                frequency: spec.freq as u32,
                generator: system.borrow().apu_ref().borrow().chn_3.clone(),
            })
            .map_err(|e| e.to_string())?;

        let chn_4 = audio
            .open_playback(None, &desired_spec, |spec| Driver {
                frequency: spec.freq as u32,
                generator: system.borrow().apu_ref().borrow().chn_4.clone(),
            })
            .map_err(|e| e.to_string())?;

        chn_1.resume();
        chn_2.resume();
        chn_3.resume();
        chn_4.resume();

        let cpu = Rc::new(RefCell::new(Cpu::new(
            Rc::clone(&system) as Rc<RefCell<dyn library::bus::Addressable>>
        )));
        int.borrow_mut().cpu = Rc::downgrade(&cpu);

        let clock = Rc::new(RefCell::new(Clock::new(
            Rc::clone(system.borrow().ppu_ref()),
            Rc::clone(system.borrow().io_ref()),
            Rc::clone(system.borrow().int_ref()),
        )));

        let mut event_pump = ctx.event_pump().unwrap();

        'running: loop {
            for event in event_pump.poll_iter() {
                use sdl2::event::Event;
                use sdl2::keyboard::Keycode;

                let mut input = input.borrow_mut();

                match event {
                    Event::Quit { .. } => break 'running,
                    Event::KeyDown {
                        keycode: Some(keycode),
                        ..
                    } => match keycode {
                        Keycode::S => input.start = true,
                        Keycode::A => input.select = true,
                        Keycode::Z => input.b = true,
                        Keycode::X => input.a = true,
                        Keycode::Up => input.up = true,
                        Keycode::Down => input.down = true,
                        Keycode::Left => input.left = true,
                        Keycode::Right => input.right = true,
                        _ => {}
                    },
                    Event::KeyUp {
                        keycode: Some(keycode),
                        ..
                    } => match keycode {
                        Keycode::S => input.start = false,
                        Keycode::A => input.select = false,
                        Keycode::Z => input.b = false,
                        Keycode::X => input.a = false,
                        Keycode::Up => input.up = false,
                        Keycode::Down => input.down = false,
                        Keycode::Left => input.left = false,
                        Keycode::Right => input.right = false,
                        _ => {}
                    },
                    _ => {}
                }
            }

            'cycle: loop {
                let cycles = cpu.borrow_mut().execute().unwrap();

                if clock.borrow_mut().clock(cycles) {
                    break 'cycle;
                }
            }

            let (w, h) = canvas.output_size()?;

            let scale_x = (w as f64) / (surface::SCREEN_WIDTH as f64);
            let scale_y = (h as f64) / (surface::SCREEN_HEIGHT as f64);
            let scale = scale_x.min(scale_y);

            let fit_w = (surface::SCREEN_WIDTH as f64) * scale;
            let fit_h = (surface::SCREEN_HEIGHT as f64) * scale;

            canvas.clear();
            canvas.copy(
                &*texture.borrow_mut(),
                None,
                Some(sdl2::rect::Rect::new(
                    (w as i32 - fit_w as i32) / 2,
                    (h as i32 - fit_h as i32) / 2,
                    fit_w as u32,
                    fit_h as u32,
                )),
            )?;
            canvas.present();
        }
    }

    Ok(())
}
