use crate::{io::IO, ppu::Ppu};
use std::{
    cell::RefCell,
    rc::Rc,
    time::{Duration, Instant},
};

const CYCLES_PER_SCANLINE: u32 = (80 + 376) / 4;
const CYCLES_PER_FRAME: u32 = 70224 / 4;
const SECONDS_PER_FRAME: f64 = CYCLES_PER_FRAME as f64 / 1_047_500.0f64;

pub struct Clock {
    ppu: Rc<RefCell<Ppu>>,
    io: Rc<RefCell<IO>>,
    cycles: u32,
    timestamp: Instant,
}

impl Clock {
    pub fn new(ppu: Rc<RefCell<Ppu>>, io: Rc<RefCell<IO>>) -> Self {
        Self {
            ppu,
            io,
            cycles: 0,
            timestamp: Instant::now(),
        }
    }

    pub fn clock(&mut self, m_cycles: u32) -> bool {
        if self.cycles % CYCLES_PER_SCANLINE > (self.cycles + m_cycles) % CYCLES_PER_SCANLINE {
            self.ppu.borrow_mut().scanline();
        }

        self.cycles += m_cycles;
        self.io.borrow_mut().clock(m_cycles);

        if self.cycles >= CYCLES_PER_FRAME {
            self.cycles -= CYCLES_PER_FRAME;

            let frame = Duration::from_secs_f64(SECONDS_PER_FRAME);
            let elapsed = self.timestamp.elapsed();

            if elapsed < frame {
                std::thread::sleep(frame - elapsed);
            }

            self.timestamp = Instant::now();

            true
        } else {
            false
        }
    }
}
