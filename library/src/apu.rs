use crate::bus::Addressable;
use std::time::{Duration, Instant};

pub const MASTER_CONTROL_ADDR: u16 = 0xff26;
pub const CHN_PAN_ADDR: u16 = 0xff25;
pub const MASTER_VOL_ADDR: u16 = 0xff25;
pub const PULSE_CHN_SWEEP_OFF: u16 = 0;
pub const PULSE_CHN_TIMER_OFF: u16 = 1;
pub const PULSE_CHN_ENVELOPE_OFF: u16 = 2;
pub const PULSE_CHN_PER_LO_OFF: u16 = 3;
pub const PULSE_CHN_PER_HI_OFF: u16 = 4;
pub const NOISE_CHN_TIMER_ADDR: u16 = 0xff20;
pub const NOISE_CHN_ENVELOPE_ADDR: u16 = 0xff21;
pub const NOISE_CHN_FREQ_ADDR: u16 = 0xff22;
pub const NOISE_CHN_CONTROL_ADDR: u16 = 0xff23;

#[derive(Default)]
struct PulseChannel<const BASE: u16> {
    enable: bool,
    left: bool,
    right: bool,
    pace: u8,
    additive: bool,
    step: u8,
    period_set: u16,
    period: u16,
    duty_cycle: u8,
    length_enable: bool,
    timer: Option<Instant>,
    volume: u8,
    env_increase: bool,
    sweep_pace: u8,
    acc1: Duration,
    acc2: Duration,
}

impl<const BASE: u16> PulseChannel<BASE> {
    pub fn update(&mut self, delta: Duration) {
        if !self.enable {
            return;
        }

        if let Some(instant) = self.timer {
            if self.length_enable && instant.checked_duration_since(Instant::now()).is_some() {
                self.enable = false;
                self.timer = None;
            }
        }

        {
            let mut delta = delta.clone() + self.acc1;
            let hz = Duration::from_secs_f64((self.pace as f64) / 128.0);

            while delta > hz {
                delta -= hz;
                self.iter();
            }

            self.acc1 = delta;
        }

        if self.sweep_pace != 0 {
            let mut delta = delta.clone() + self.acc2;
            let hz = Duration::from_secs_f64((self.sweep_pace as f64) / 64.0);

            while delta > hz {
                delta -= hz;

                if self.env_increase {
                    self.volume += 1;
                    if self.volume > 0xf {
                        self.volume = 0;
                    }
                } else {
                    self.volume = self.volume.wrapping_sub(1);
                    if self.volume > 0xf {
                        self.volume = 0xf;
                    }
                }
            }

            self.acc2 = delta;
        }
    }

    fn iter(&mut self) {
        let delta = self.period / 2u16.pow(self.step as u32);

        // Didn't want to deal with the unsigned cast, okay?
        self.period = if self.additive {
            self.period + delta
        } else {
            self.period - delta
        };

        if self.period > 0x7ff {
            self.enable = false;
        }
    }
}

impl<const BASE: u16> Addressable for PulseChannel<BASE> {
    fn read(&mut self, addr: u16) -> Option<u8> {
        match addr.wrapping_sub(BASE) {
            PULSE_CHN_PER_HI_OFF => Some(if self.length_enable { 0x40 } else { 0 }),
            _ => None,
        }
    }

    fn write(&mut self, addr: u16, value: u8) -> Option<()> {
        match addr.wrapping_sub(BASE) {
            PULSE_CHN_SWEEP_OFF => {
                self.pace = (value >> 4) & 7;
                self.additive = (value & 4) == 0;
                self.step = value & 7;
            }
            PULSE_CHN_TIMER_OFF => {
                self.duty_cycle = value >> 6;
                self.timer.replace(
                    Instant::now()
                        + Duration::from_secs_f64((64.0 - (value & 0x3f) as f64) / 256.0),
                );
            }
            PULSE_CHN_ENVELOPE_OFF => {
                self.volume = value >> 4;
                self.env_increase = (value & 8) != 0;
                self.sweep_pace = value & 7;

                if value & !7 == 0 {
                    self.enable = false;
                }
            }
            PULSE_CHN_PER_LO_OFF => self.period_set = (self.period_set & 0xff00) | (value as u16),
            PULSE_CHN_PER_HI_OFF => {
                self.period_set = (self.period_set & 0x00ff) | ((value & 7) as u16);
                self.length_enable = (value & 0x40) != 0;

                if value & 0x80 != 0 {
                    self.enable = true;
                    self.period = self.period_set;

                    if self.length_enable {
                        self.timer = None;
                    }
                }
            }
            _ => return None,
        }

        Some(())
    }
}

#[derive(Default)]
struct NoiseChannel {
    enable: bool,
    length_enable: bool,
    timer: Option<Instant>,
    volume: u8,
    left: bool,
    right: bool,
    env_increase: bool,
    sweep_pace: u8,
    lfsr_short: bool,
    lfsr: u16,
    period: Duration,
    acc1: Duration,
    acc2: Duration,
}

impl NoiseChannel {
    pub fn update(&mut self, delta: Duration) {
        if !self.enable {
            return;
        }

        if let Some(instant) = self.timer {
            if self.length_enable && instant.checked_duration_since(Instant::now()).is_some() {
                self.enable = false;
                self.timer = None;
            }
        }

        {
            let mut delta = delta.clone() + self.acc1;

            while delta > self.period {
                delta -= self.period;
                self.iter();
            }

            self.acc1 = delta;
        }

        if self.sweep_pace != 0 {
            let mut delta = delta.clone() + self.acc2;
            let hz = Duration::from_secs_f64((self.sweep_pace as f64) / 64.0);

            while delta > hz {
                delta -= hz;

                if self.env_increase {
                    self.volume += 1;
                    if self.volume > 0xf {
                        self.volume = 0;
                    }
                } else {
                    self.volume = self.volume.wrapping_sub(1);
                    if self.volume > 0xf {
                        self.volume = 0xf;
                    }
                }
            }

            self.acc2 = delta;
        }
    }

    fn iter(&mut self) {
        self.lfsr = (self.lfsr & !(1 << 15)) | (((self.lfsr & 1) ^ ((self.lfsr >> 1) & 1)) << 15);

        if self.lfsr_short {
            self.lfsr = (self.lfsr & !0x80) | ((self.lfsr >> 8) & 0x80);
        }

        self.lfsr >>= 1;

        if (self.lfsr & 1) != 0 {
            unimplemented!();
        } else {
            unimplemented!();
        }
    }
}

impl Addressable for NoiseChannel {
    fn read(&mut self, addr: u16) -> Option<u8> {
        match addr {
            NOISE_CHN_CONTROL_ADDR => Some(if self.length_enable { 0x40 } else { 0 }),
            _ => None,
        }
    }

    fn write(&mut self, addr: u16, value: u8) -> Option<()> {
        match addr {
            NOISE_CHN_TIMER_ADDR => {
                self.timer.replace(
                    Instant::now()
                        + Duration::from_secs_f64((64.0 - (value & 0x3f) as f64) / 256.0),
                );
            }
            NOISE_CHN_ENVELOPE_ADDR => {
                self.volume = value >> 4;
                self.env_increase = value & 8 != 0;
                self.sweep_pace = value & 7;

                if value & !7 == 0 {
                    self.enable = false;
                }
            }
            NOISE_CHN_FREQ_ADDR => {
                let shift = value >> 4;
                let divider = value & 7;

                self.lfsr_short = (value & 4) != 0;

                self.period = Duration::from_secs_f64(
                    262144.0 / ((divider as f64) * (2u32.pow(shift as u32)) as f64),
                );
            }
            NOISE_CHN_CONTROL_ADDR => {
                self.length_enable = value & 0x40 != 0;

                if value & 0x80 != 0 {
                    self.enable = true;
                    self.lfsr = 0;

                    if self.length_enable {
                        self.timer = None;
                    }
                }
            }
            _ => return None,
        }

        Some(())
    }
}

pub struct Apu {
    enable: bool,
    chn_1: PulseChannel<0xff10>,
    chn_2: PulseChannel<0xff15>,
    chn_3: PulseChannel<0xff10>,
    chn_4: PulseChannel<0xff10>,
    left_volume: u8,
    right_volume: u8,
    then: Instant,
}

impl Apu {
    pub fn update(&mut self) {
        let now = Instant::now();
        let delta = now - self.then;
        self.then = now;

        self.chn_1.update(delta);
    }
}

impl Addressable for Apu {
    fn read(&mut self, addr: u16) -> Option<u8> {
        match addr {
            MASTER_CONTROL_ADDR => Some(
                if self.enable { 0x80 } else { 0 }
                    | if self.chn_4.enable { 8 } else { 0 }
                    | if self.chn_3.enable { 4 } else { 0 }
                    | if self.chn_2.enable { 2 } else { 0 }
                    | if self.chn_1.enable { 1 } else { 0 },
            ),
            _ => self
                .chn_1
                .read(addr)
                .or_else(|| self.chn_2.read(addr))
                .or_else(|| self.chn_3.read(addr))
                .or_else(|| self.chn_4.read(addr)),
        }
    }

    fn write(&mut self, addr: u16, value: u8) -> Option<()> {
        match addr {
            MASTER_CONTROL_ADDR => self.enable = (value & 0x80) != 0,
            CHN_PAN_ADDR => {
                self.chn_4.left = (value & (1 << 7)) != 0;
                self.chn_3.left = (value & (1 << 6)) != 0;
                self.chn_2.left = (value & (1 << 5)) != 0;
                self.chn_1.left = (value & (1 << 4)) != 0;
                self.chn_4.right = (value & (1 << 3)) != 0;
                self.chn_3.right = (value & (1 << 2)) != 0;
                self.chn_2.right = (value & (1 << 1)) != 0;
                self.chn_1.right = (value & (1 << 0)) != 0;
            }
            MASTER_VOL_ADDR => {
                self.left_volume = (value >> 4) & 7;
                self.right_volume = value & 7;
            }
            _ => {
                return self
                    .chn_1
                    .write(addr, value)
                    .or_else(|| self.chn_2.write(addr, value))
                    .or_else(|| self.chn_3.write(addr, value))
                    .or_else(|| self.chn_4.write(addr, value))
            }
        }

        Some(())
    }
}
