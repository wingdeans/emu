use crate::{bus::Bus, error::Result};
use std::cmp::max;

pub const BANK_COUNT: u32 = 8;
pub const BANK_SIZE: usize = 1024 * 4;

const BANK_0_X_MASK: u16 = 0x1000;
const BANK_ADDR_MASK: u16 = 0x0fff;

pub struct Wram {
    banks: [[u8; BANK_SIZE]; BANK_COUNT as usize],
    select: u32,
}

impl Wram {
    pub fn set_bank(&mut self, bank: u32) {
        self.select = max(1, bank % BANK_COUNT);
    }

    pub fn bank(&self) -> u32 {
        self.select
    }
}

impl Default for Wram {
    fn default() -> Self {
        Self {
            banks: [[0; BANK_SIZE]; BANK_COUNT as usize],
            select: 1,
        }
    }
}

impl Bus for Wram {
    fn read(&mut self, addr: u16) -> Result<u8> {
        let bank = if addr & BANK_0_X_MASK != 0 {
            &self.banks[self.select as usize]
        } else {
            &self.banks[0]
        };

        Ok(bank[(addr & BANK_ADDR_MASK) as usize])
    }

    fn write(&mut self, addr: u16, value: u8) -> Result<()> {
        let bank = if addr & BANK_0_X_MASK != 0 {
            &mut self.banks[self.select as usize]
        } else {
            &mut self.banks[0]
        };

        bank[(addr & BANK_ADDR_MASK) as usize] = value;
        Ok(())
    }
}
