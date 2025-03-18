use crate::ppu::Ppu;
use std::{
    cell::RefCell,
    rc::Rc,
    time::{Duration, Instant},
};

const CYCLES_PER_SCANLINE: u32 = (80 + 376) / 4;
const CYCLES_PER_FRAME: u32 = 70224 / 4;

pub struct Clock {
    ppu: Rc<RefCell<Ppu>>,
    cycles: u32,
    timestamp: Instant,
}

impl Clock {
    pub fn new(ppu: Rc<RefCell<Ppu>>) -> Self {
        Self {
            ppu,
            cycles: 0,
            timestamp: Instant::now(),
        }
    }

    pub fn clock(&mut self, m_cycles: u32) -> bool {
        if self.cycles % CYCLES_PER_SCANLINE > (self.cycles + m_cycles) % CYCLES_PER_SCANLINE {
            self.ppu.borrow_mut().scanline();
        }

        self.cycles += m_cycles;

        if self.cycles >= CYCLES_PER_FRAME {
            self.cycles = 0;

            let frame = Duration::from_secs_f64((self.cycles as f64) / (1_047_500.0f64));
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
