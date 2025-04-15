use crate::bus::Addressable;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub const MASTER_CONTROL_ADDR: u16 = 0xff26;
pub const CHN_PAN_ADDR: u16 = 0xff25;
pub const MASTER_VOL_ADDR: u16 = 0xff24;
pub const PULSE_CHN_SWEEP_OFF: u16 = 0;
pub const PULSE_CHN_TIMER_OFF: u16 = 1;
pub const PULSE_CHN_ENVELOPE_OFF: u16 = 2;
pub const PULSE_CHN_PER_LO_OFF: u16 = 3;
pub const PULSE_CHN_PER_HI_OFF: u16 = 4;
pub const NOISE_CHN_TIMER_ADDR: u16 = 0xff20;
pub const NOISE_CHN_ENVELOPE_ADDR: u16 = 0xff21;
pub const NOISE_CHN_FREQ_ADDR: u16 = 0xff22;
pub const NOISE_CHN_CONTROL_ADDR: u16 = 0xff23;
pub const WAVE_CHN_ENABLE_ADDR: u16 = 0xff1a;
pub const WAVE_CHN_TIMER_ADDR: u16 = 0xff1b;
pub const WAVE_CHN_OUTPUT_ADDR: u16 = 0xff1c;
pub const WAVE_CHN_PER_LO_ADDR: u16 = 0xff1d;
pub const WAVE_CHN_PER_HI_ADDR: u16 = 0xff1e;
pub const WAVE_PATTERN_RAM_BEGIN: u16 = 0xff30;
pub const WAVE_PATTERN_RAM_END: u16 = 0xff40;
pub const WAVE_PATTERN_RAM_SIZE: usize = (WAVE_PATTERN_RAM_END - WAVE_PATTERN_RAM_BEGIN) as usize;

pub trait Generator {
    fn generate(&mut self, frequency: u32, buffer: &mut [f32]);
}

#[derive(Default)]
pub struct PulseChannel<const BASE: u16> {
    pub enable: bool,
    pub left: bool,
    pub right: bool,
    pace: u8,
    additive: bool,
    step: u8,
    period_set: u16,
    period: u16,
    duty_cycle: u8,
    length_enable: bool,
    timer: Duration,
    volume: u8,
    env_increase: bool,
    sweep_pace: u8,
}

impl<const BASE: u16> Generator for PulseChannel<BASE> {
    fn generate(&mut self, frequency: u32, buffer: &mut [f32]) {
        if !self.enable {
            return;
        }

        let ratio = match self.duty_cycle & 3 {
            0b00 => 0.125,
            0b01 => 0.25,
            0b10 => 0.50,
            0b11 => 0.75,
            _ => unreachable!(),
        };

        let atom = Duration::from_secs_f64(1.0 / frequency as f64);
        let pace = Duration::from_secs_f64(self.pace as f64 / 128.0);

        let sweep_pace = if self.sweep_pace == 0 {
            None
        } else {
            Some(Duration::from_secs_f64((self.sweep_pace as f64) / 64.0))
        };

        let period = if (self.period & (1 << 10)) != 0 {
            self.period | 0xf800
        } else {
            self.period
        } as i16;

        let mut period = Duration::from_secs_f64(131072.0 / (2048.0 - (period as f64)));

        let mut duration = atom * buffer.len() as u32;
        let mut acc0 = Duration::ZERO;
        let mut acc1 = Duration::ZERO;

        let mut idx = 0;

        while duration > period {
            let items = (period.as_secs_f64() / atom.as_secs_f64()) as usize;
            let high = (items as f64 * ratio) as usize;

            for i in 0..high {
                buffer[idx + i] = (self.volume as f32) / (0b1111 as f32);
            }

            idx += items as usize;
            duration -= period;

            if self.length_enable {
                if self.timer < period {
                    self.enable = false;
                    return;
                } else {
                    self.timer -= period;
                }
            }

            acc0 += period;
            acc1 += period;

            if acc0 > pace {
                let delta = self.period / 2u16.pow(self.step as u32);

                self.period = if self.additive {
                    self.period + delta
                } else {
                    self.period - delta
                };

                if self.period > 0x7ff {
                    self.enable = false;
                }

                let local = if (self.period & (1 << 10)) != 0 {
                    self.period | 0xf800
                } else {
                    self.period
                } as i16;

                period = Duration::from_secs_f64(131072.0 / (2048.0 - (local as f64)));

                acc0 -= pace;
            }

            if let Some(sweep_pace) = sweep_pace {
                if acc1 > sweep_pace {
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

                    acc1 -= sweep_pace;
                }
            }
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
                self.timer = Duration::from_secs_f64((64.0 - (value & 0x3f) as f64) / 256.0);
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
                        self.timer = Duration::ZERO;
                    }
                }
            }
            _ => return None,
        }

        Some(())
    }
}

#[derive(Default)]
pub struct NoiseChannel {
    pub enable: bool,
    pub left: bool,
    pub right: bool,
    length_enable: bool,
    timer: Duration,
    volume: u8,
    env_increase: bool,
    sweep_pace: u8,
    lfsr_short: bool,
    lfsr: u16,
    period: Duration,
}

impl Generator for NoiseChannel {
    fn generate(&mut self, frequency: u32, buffer: &mut [f32]) {
        if !self.enable {
            return;
        }

        let atom = Duration::from_secs_f64(1.0 / frequency as f64);

        let sweep_pace = if self.sweep_pace == 0 {
            None
        } else {
            Some(Duration::from_secs_f64((self.sweep_pace as f64) / 64.0))
        };

        let mut acc0 = Duration::ZERO;
        let mut acc1 = Duration::ZERO;

        for i in 0..buffer.len() {
            buffer[i] = (self.lfsr & 1) as f32 * ((self.volume as f32) / (0b1111 as f32));

            if self.length_enable {
                if self.timer < atom {
                    self.enable = false;
                    return;
                } else {
                    self.timer -= atom;
                }
            }

            acc0 += atom;
            acc1 += atom;

            if acc0 > self.period {
                self.lfsr =
                    (self.lfsr & !(1 << 15)) | (((self.lfsr & 1) ^ ((self.lfsr >> 1) & 1)) << 15);

                if self.lfsr_short {
                    self.lfsr = (self.lfsr & !0x80) | ((self.lfsr >> 8) & 0x80);
                }

                self.lfsr >>= 1;

                acc0 -= self.period;
            }

            if let Some(sweep_pace) = sweep_pace {
                if acc1 > sweep_pace {
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

                    acc1 -= sweep_pace;
                }
            }
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
                self.timer = Duration::from_secs_f64((64.0 - (value & 0x3f) as f64) / 256.0);
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
                        self.timer = Duration::ZERO;
                    }
                }
            }
            _ => return None,
        }

        Some(())
    }
}

#[derive(Default)]
pub struct WaveChannel {
    pub enable: bool,
    pub left: bool,
    pub right: bool,
    length_enable: bool,
    timer: Duration,
    output_level: u8,
    period_set: u16,
    period: u16,
    ram: [u8; WAVE_PATTERN_RAM_SIZE],
}

impl Generator for WaveChannel {
    fn generate(&mut self, frequency: u32, buffer: &mut [f32]) {
        if !self.enable {
            return;
        }

        let atom = Duration::from_secs_f64(1.0 / frequency as f64);

        let period = if (self.period & (1 << 10)) != 0 {
            self.period | 0xf800
        } else {
            self.period
        } as i16;

        let period = Duration::from_secs_f64(65536.0 / (2048.0 - (period as f64)));

        let mut duration = atom * buffer.len() as u32;
        let mut idx = 0;

        while duration > period {
            for i in 0..32 {
                let value = self.ram[i / 2];
                let sample = if (i & 1) != 0 {
                    value & 0x0f
                } else {
                    value >> 4
                };

                let sample = match self.output_level & 3 {
                    0b00 => 0,
                    0b01 => sample,
                    0b10 => sample >> 1,
                    0b11 => sample >> 2,
                    _ => unreachable!(),
                };

                buffer[idx + i] = (sample as f32) / (0b1111 as f32);
            }

            idx += 32;
            duration -= period;

            if self.length_enable {
                if self.timer < period {
                    self.enable = false;
                    return;
                } else {
                    self.timer -= period;
                }
            }
        }
    }
}

impl Addressable for WaveChannel {
    fn read(&mut self, addr: u16) -> Option<u8> {
        match addr {
            WAVE_PATTERN_RAM_BEGIN..WAVE_PATTERN_RAM_END => {
                Some(self.ram[(addr - WAVE_PATTERN_RAM_BEGIN) as usize])
            }
            _ => None,
        }
    }

    fn write(&mut self, addr: u16, value: u8) -> Option<()> {
        match addr {
            WAVE_PATTERN_RAM_BEGIN..WAVE_PATTERN_RAM_END => {
                self.ram[(addr - WAVE_PATTERN_RAM_BEGIN) as usize] = value
            }
            WAVE_CHN_ENABLE_ADDR => self.enable = (value & 0x80) != 0,
            WAVE_CHN_TIMER_ADDR => {
                self.timer = Duration::from_secs_f64((256.0 - (value & 0x3f) as f64) / 256.0);
            }
            WAVE_CHN_OUTPUT_ADDR => self.output_level = (value >> 5) & 3,
            WAVE_CHN_PER_LO_ADDR => self.period_set = (self.period_set & 0xff00) | (value as u16),
            WAVE_CHN_PER_HI_ADDR => {
                self.period_set = (self.period_set & 0x00ff) | ((value & 7) as u16);
                self.length_enable = (value & 0x40) != 0;

                if value & 0x80 != 0 {
                    self.enable = true;
                    self.period = self.period_set;

                    if self.length_enable {
                        self.timer = Duration::ZERO;
                    }
                }
            }
            _ => return None,
        }

        Some(())
    }
}

#[derive(Default)]
pub struct Apu {
    enable: bool,
    pub chn_1: Arc<Mutex<PulseChannel<0xff10>>>,
    pub chn_2: Arc<Mutex<PulseChannel<0xff15>>>,
    pub chn_3: Arc<Mutex<WaveChannel>>,
    pub chn_4: Arc<Mutex<NoiseChannel>>,
    left_volume: u8,
    right_volume: u8,
}

impl Addressable for Apu {
    fn read(&mut self, addr: u16) -> Option<u8> {
        let mut chn_1 = self.chn_1.lock().unwrap();
        let mut chn_2 = self.chn_2.lock().unwrap();
        let mut chn_3 = self.chn_3.lock().unwrap();
        let mut chn_4 = self.chn_4.lock().unwrap();

        match addr {
            MASTER_CONTROL_ADDR => Some(
                if self.enable { 0x80 } else { 0 }
                    | if chn_4.enable { 8 } else { 0 }
                    | if chn_3.enable { 4 } else { 0 }
                    | if chn_2.enable { 2 } else { 0 }
                    | if chn_1.enable { 1 } else { 0 },
            ),
            _ => chn_1
                .read(addr)
                .or_else(|| chn_2.read(addr))
                .or_else(|| chn_3.read(addr))
                .or_else(|| chn_4.read(addr)),
        }
    }

    fn write(&mut self, addr: u16, value: u8) -> Option<()> {
        let mut chn_1 = self.chn_1.lock().unwrap();
        let mut chn_2 = self.chn_2.lock().unwrap();
        let mut chn_3 = self.chn_3.lock().unwrap();
        let mut chn_4 = self.chn_4.lock().unwrap();

        match addr {
            MASTER_CONTROL_ADDR => self.enable = (value & 0x80) != 0,
            CHN_PAN_ADDR => {
                chn_4.left = (value & (1 << 7)) != 0;
                chn_3.left = (value & (1 << 6)) != 0;
                chn_2.left = (value & (1 << 5)) != 0;
                chn_1.left = (value & (1 << 4)) != 0;
                chn_4.right = (value & (1 << 3)) != 0;
                chn_3.right = (value & (1 << 2)) != 0;
                chn_2.right = (value & (1 << 1)) != 0;
                chn_1.right = (value & (1 << 0)) != 0;
            }
            MASTER_VOL_ADDR => {
                self.left_volume = (value >> 4) & 7;
                self.right_volume = value & 7;
            }
            _ => {
                return chn_1
                    .write(addr, value)
                    .or_else(|| chn_2.write(addr, value))
                    .or_else(|| chn_3.write(addr, value))
                    .or_else(|| chn_4.write(addr, value))
            }
        }

        Some(())
    }
}
