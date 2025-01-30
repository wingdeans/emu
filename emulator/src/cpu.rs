use crate::error::{Error, Result};
use library::memory;

pub struct Cpu {
    af: u16,
    bc: u16,
    de: u16,
    hl: u16,
    sp: u16,
    pc: u16,
    ime: bool,
    halted: bool,
    stopped: bool,
}

macro_rules! ins {
    ($ins:ident, r8) => {
        ($ins >> 3) & 0b111
    };
    ($ins:ident, r8l) => {
        $ins & 0b111
    };
    ($ins:ident, r16) => {
        ($ins >> 4) & 0b11
    };
    ($ins:ident, cond) => {
        ($ins >> 3) & 0b11
    };
    ($ins:ident, b3) => {
        ($ins >> 3) & 0b111
    };
    ($ins:ident, tgt3) => {
        ($ins >> 3) & 0b111
    };
}

fn add8(a: u8, b: u8, c: bool) -> (u8, bool, bool) {
    let (sum, c1) = a.overflowing_add(b);
    let (result, c2) = sum.overflowing_add(if c { 1 } else { 0 });

    let half = (a & 0x0f)
        .wrapping_add(b & 0x0f)
        .wrapping_add(if c { 1 } else { 0 });

    (result, c1 || c2, (half & 0xf0) != 0)
}

fn add16(a: u16, b: u16, c: bool) -> (u16, bool, bool) {
    let (value, c1) = a.overflowing_add(b);
    let (result, c2) = value.overflowing_add(if c { 1 } else { 0 });

    let half = (a & 0x0fff)
        .wrapping_add(b & 0x0fff)
        .wrapping_add(if c { 1 } else { 0 });

    (result, c1 || c2, (half & 0xf000) != 0)
}

fn sub8(a: u8, b: u8, c: bool) -> (u8, bool, bool) {
    let (value, c1) = a.overflowing_sub(b);
    let (result, c2) = value.overflowing_sub(if c { 1 } else { 0 });

    let half = (a & 0xf0).wrapping_sub(b & 0xf0); // WARN Not sure about this one

    (result, c1 || c2, (half & 0x0f) != 0)
}

fn sub16(a: u16, b: u16, c: bool) -> (u16, bool, bool) {
    let (value, c1) = a.overflowing_sub(b);
    let (result, c2) = value.overflowing_sub(if c { 1 } else { 0 });

    let half = (a & 0xf000).wrapping_sub(b & 0xf000); // WARN Not sure about this one

    (result, c1 || c2, (half & 0x0fff) != 0)
}

#[allow(dead_code)]
impl Cpu {
    fn get_flag(&self, bit: u8) -> bool {
        (self.af & (1 << bit)) != 0
    }

    fn set_flag(&mut self, bit: u8, value: bool) {
        if value {
            self.af |= 1 << bit;
        } else if self.get_flag(bit) {
            self.af ^= 1 << bit;
        }
    }

    fn zero(&self) -> bool {
        self.get_flag(7)
    }

    fn set_zero(&mut self, value: bool) {
        self.set_flag(7, value);
    }

    fn sub(&self) -> bool {
        self.get_flag(6)
    }

    fn set_sub(&mut self, value: bool) {
        self.set_flag(6, value);
    }

    fn half_carry(&self) -> bool {
        self.get_flag(5)
    }

    fn set_half_carry(&mut self, value: bool) {
        self.set_flag(5, value);
    }

    fn carry(&self) -> bool {
        self.get_flag(4)
    }

    fn set_carry(&mut self, value: bool) {
        self.set_flag(4, value);
    }

    fn cond(&self, cond: u8) -> Result<bool> {
        Ok(match cond {
            0 => !self.zero(),
            1 => self.zero(),
            2 => !self.carry(),
            3 => self.carry(),
            _ => return Err(Error::InvalidIndexWidth(2, cond)),
        })
    }

    fn r8(&self, index: u8) -> Result<u8> {
        Ok(match index {
            0 => (self.bc >> 8) as u8,
            1 => (self.bc & 0xff) as u8,
            2 => (self.de >> 8) as u8,
            3 => (self.de & 0xff) as u8,
            4 => (self.hl >> 8) as u8,
            5 => (self.hl & 0xff) as u8,
            6 => return Err(Error::NotImplemented), // ERROR
            7 => (self.af >> 8) as u8,
            _ => return Err(Error::InvalidIndexWidth(3, index)),
        })
    }

    fn set_r8(&mut self, index: u8, value: u8) -> Result<()> {
        match index {
            0 => self.bc = ((value as u16) << 8) | (self.bc & 0xff),
            1 => self.bc = (self.bc & 0xff00) | (value as u16),
            2 => self.de = ((value as u16) << 8) | (self.de & 0xff),
            3 => self.de = (self.de & 0xff00) | (value as u16),
            4 => self.hl = ((value as u16) << 8) | (self.hl & 0xff),
            5 => self.hl = (self.hl & 0xff00) | (value as u16),
            6 => return Err(Error::NotImplemented), // ERROR
            7 => self.af = ((value as u16) << 8) | (self.af & 0xff),
            _ => return Err(Error::InvalidIndexWidth(3, index)),
        }

        Ok(())
    }

    fn r16(&self, ins: u8) -> u16 {
        match ins!(ins, r16) {
            0 => self.bc,
            1 => self.de,
            2 => self.hl,
            3 => self.af,
            _ => unreachable!(),
        }
    }

    fn set_r16(&mut self, ins: u8, value: u16) {
        match ins!(ins, r16) {
            0 => self.bc = value,
            1 => self.de = value,
            2 => self.hl = value,
            3 => self.sp = value,
            _ => unreachable!(),
        }
    }

    fn a(&self) -> u8 {
        (self.af >> 8) as u8
    }

    fn set_a(&mut self, value: u8) {
        self.af = ((value as u16) << 8) | (self.af & 0xff);
    }

    fn imm8(&mut self) -> u8 {
        let value = memory::read(self.pc);
        self.pc += 1;
        value
    }

    fn imm16(&mut self) -> u16 {
        let lo = memory::read(self.pc + 0);
        let hi = memory::read(self.pc + 1);

        self.pc += 2;
        ((hi as u16) << 8) | (lo as u16)
    }

    fn stack_push(&mut self, value: u16) {
        memory::write(self.sp - 1, (value >> 8) as u8);
        memory::write(self.sp - 2, (value & 0xff) as u8);
        self.sp -= 2;
    }

    fn stack_pop(&mut self) -> u16 {
        let lo = memory::read(self.sp + 0);
        let hi = memory::read(self.sp + 1);
        self.sp += 2;

        ((hi as u16) << 8) | (lo as u16)
    }

    fn nop(&mut self, _ins: u8) -> Result<u32> {
        Ok(1)
    }

    fn ld_r16_imm16(&mut self, ins: u8) -> Result<u32> {
        let value = self.imm16();
        self.set_r16(ins, value);
        Ok(3)
    }

    fn ld_r16mem_a(&mut self, ins: u8) -> Result<u32> {
        let addr = match ins!(ins, r16) {
            0 => self.bc,
            1 => self.de,
            2 => {
                self.hl += 1;
                self.hl - 1
            }
            3 => {
                self.hl -= 1;
                self.hl + 1
            }
            _ => unreachable!(),
        };

        memory::write(addr, self.a());
        Ok(2)
    }

    fn ld_a_r16mem(&mut self, ins: u8) -> Result<u32> {
        let addr = match ins!(ins, r16) {
            0 => self.bc,
            1 => self.de,
            2 => {
                self.hl += 1;
                self.hl - 1
            }
            3 => {
                self.hl -= 1;
                self.hl + 1
            }
            _ => unreachable!(),
        };

        self.set_a(memory::read(addr));
        Ok(2)
    }

    fn ld_imm16_sp(&mut self, _ins: u8) -> Result<u32> {
        let addr = self.imm16();
        memory::write(addr + 0, self.sp as u8);
        memory::write(addr + 1, (self.sp >> 8) as u8);
        Ok(5)
    }

    fn inc_r16(&mut self, ins: u8) -> Result<u32> {
        let value = self.r16(ins).wrapping_add(1);
        self.set_r16(ins, value);
        Ok(2)
    }

    fn dec_r16(&mut self, ins: u8) -> Result<u32> {
        let value = self.r16(ins).wrapping_sub(1);
        self.set_r16(ins, value);
        Ok(2)
    }

    fn add_hl_r16(&mut self, ins: u8) -> Result<u32> {
        let (value, c, hc) = add16(self.hl, self.r16(ins), self.carry());

        self.set_sub(false);
        self.set_carry(c);
        self.set_half_carry(hc);

        self.hl = value;
        Ok(2)
    }

    fn inc_r8(&mut self, ins: u8) -> Result<u32> {
        let value = self.r8(ins!(ins, r8))?;
        let (result, _c, hc) = add8(value, 1, false);

        self.set_zero(value == 0);
        self.set_sub(false);
        self.set_half_carry(hc);

        self.set_r8(ins!(ins, r8), result)?;
        Ok(1)
    }

    fn inc_hl(&mut self, _ins: u8) -> Result<u32> {
        let value = memory::read(self.hl);
        let (result, _c, hc) = add8(value, 1, false);

        self.set_zero(value == 0);
        self.set_sub(false);
        self.set_half_carry(hc);

        memory::write(self.hl, result);
        Ok(3)
    }

    fn dec_r8(&mut self, ins: u8) -> Result<u32> {
        let value = self.r8(ins!(ins, r8))?;
        let (result, _c, hc) = sub8(value, 1, false);

        self.set_zero(value == 0);
        self.set_sub(true);
        self.set_half_carry(hc);

        self.set_r8(ins!(ins, r8), result)?;
        Ok(1)
    }

    fn dec_hl(&mut self, _ins: u8) -> Result<u32> {
        let value = memory::read(self.hl);
        let (result, _c, hc) = sub8(value, 1, false);

        self.set_zero(value == 0);
        self.set_sub(true);
        self.set_half_carry(hc);

        memory::write(self.hl, result);
        Ok(3)
    }

    fn ld_r8_imm8(&mut self, ins: u8) -> Result<u32> {
        let value = self.imm8();
        self.set_r8(ins!(ins, r8), value)?;
        Ok(2)
    }

    fn ld_hl_imm8(&mut self, _ins: u8) -> Result<u32> {
        let value = self.imm8();
        memory::write(self.hl, value);
        Ok(3)
    }

    fn rlca(&mut self, _ins: u8) -> Result<u32> {
        let value = self.a().rotate_left(1);

        self.set_carry((value & 1) != 0);
        self.set_zero(false);
        self.set_sub(false);
        self.set_half_carry(false);

        Ok(1)
    }

    fn rrca(&mut self, _ins: u8) -> Result<u32> {
        let value = self.a().rotate_right(1);

        self.set_carry((value & 1) != 0);
        self.set_zero(false);
        self.set_sub(false);
        self.set_half_carry(false);

        Ok(1)
    }

    fn rla(&mut self, _ins: u8) -> Result<u32> {
        let value = self.a();
        let bit = if self.carry() { 1 } else { 0 };

        self.set_carry((value & 0x80) != 0);
        self.set_zero(false);
        self.set_sub(false);
        self.set_half_carry(false);

        self.set_a((value << 1) | bit);
        Ok(1)
    }

    fn rra(&mut self, _ins: u8) -> Result<u32> {
        let value = self.a();
        let bit = if self.carry() { 1 } else { 0 };

        self.set_carry((value & 1) != 0);
        self.set_zero(false);
        self.set_sub(false);
        self.set_half_carry(false);

        self.set_a((bit << 7) | (value >> 1));
        Ok(1)
    }

    fn daa(&mut self, _ins: u8) -> Result<u32> {
        let value: u8;

        if self.sub() {
            let mut adjustment = 0;

            if self.half_carry() {
                adjustment += 6;
            }

            if self.carry() {
                adjustment += 0x60;
            }

            value = self.a() - adjustment;
            self.set_carry(value > adjustment);
        } else {
            let mut adjustment = 0;

            if self.half_carry() || (self.a() & 0xf) > 9 {
                adjustment += 6;
            }

            if self.carry() || (self.a() > 0x9f) {
                adjustment += 0x60;
            }

            let carry: bool;
            (value, carry) = self.a().overflowing_add(adjustment);
            self.set_carry(carry);
        }

        self.set_a(value);
        self.set_zero(value == 0);
        self.set_half_carry(false);

        Ok(1)
    }

    fn cpl(&mut self, _ins: u8) -> Result<u32> {
        self.set_a(!self.a());
        self.set_sub(true);
        self.set_half_carry(true);
        Ok(1)
    }

    fn scf(&mut self, _ins: u8) -> Result<u32> {
        self.set_sub(false);
        self.set_half_carry(false);
        self.set_carry(true);
        Ok(1)
    }

    fn ccf(&mut self, _ins: u8) -> Result<u32> {
        self.set_sub(false);
        self.set_half_carry(false);
        self.set_carry(!self.carry());
        Ok(1)
    }

    fn jr(&mut self, _ins: u8) -> Result<u32> {
        let value = self.imm8();
        self.pc = self.pc.wrapping_add_signed(value as i16);
        Ok(3)
    }

    fn jr_cond(&mut self, ins: u8) -> Result<u32> {
        let cond = self.cond(ins!(ins, cond))?;
        let value = self.imm8();

        if cond {
            self.pc = self.pc.wrapping_add_signed(value as i16);
            Ok(3)
        } else {
            Ok(2)
        }
    }

    fn stop(&mut self, _ins: u8) -> Result<u32> {
        self.pc += 1;
        self.stopped = true;
        Ok(0)
    }

    fn ld_r8_r8(&mut self, ins: u8) -> Result<u32> {
        let dest = ins!(ins, r8);
        let src = ins!(ins, r8l);

        let value = self.r8(src)?;
        self.set_r8(dest, value)?;

        Ok(1)
    }

    fn add_a_r8(&mut self, ins: u8) -> Result<u32> {
        let value = self.r8(ins!(ins, r8l))?;
        let (result, c, hc) = add8(self.a(), value, false);

        self.set_zero(result == 0);
        self.set_sub(false);
        self.set_half_carry(hc);
        self.set_carry(c);

        self.set_a(result);
        Ok(1)
    }

    fn add_a_hl(&mut self, _ins: u8) -> Result<u32> {
        let value = memory::read(self.hl);
        let (result, c, hc) = add8(self.a(), value, false);

        self.set_zero(result == 0);
        self.set_sub(false);
        self.set_half_carry(hc);
        self.set_carry(c);

        self.set_a(result);
        Ok(2)
    }

    fn adc_a_r8(&mut self, ins: u8) -> Result<u32> {
        let value = self.r8(ins!(ins, r8l))?;
        let (result, c, hc) = add8(self.a(), value, self.carry());

        self.set_zero(result == 0);
        self.set_sub(false);
        self.set_half_carry(hc);
        self.set_carry(c);

        self.set_a(result);
        Ok(1)
    }

    fn adc_a_hl(&mut self, _ins: u8) -> Result<u32> {
        let value = memory::read(self.hl);
        let (result, c, hc) = add8(self.a(), value, self.carry());

        self.set_zero(result == 0);
        self.set_sub(false);
        self.set_half_carry(hc);
        self.set_carry(c);

        self.set_a(result);
        Ok(2)
    }

    fn sub_a_r8(&mut self, ins: u8) -> Result<u32> {
        let value = self.r8(ins!(ins, r8))?;
        let (result, c, hc) = sub8(self.a(), value, false);

        self.set_zero(result == 0);
        self.set_sub(true);
        self.set_half_carry(hc);
        self.set_carry(c);

        self.set_a(result);
        Ok(1)
    }

    fn sub_a_hl(&mut self, _ins: u8) -> Result<u32> {
        let value = memory::read(self.hl);
        let (result, c, hc) = sub8(self.a(), value, false);

        self.set_zero(result == 0);
        self.set_sub(true);
        self.set_half_carry(hc);
        self.set_carry(c);

        memory::write(self.hl, result);
        Ok(2)
    }

    fn sbc_a_r8(&mut self, ins: u8) -> Result<u32> {
        let value = self.r8(ins!(ins, r8))?;
        let (result, c, hc) = sub8(self.a(), value, self.carry());

        self.set_zero(result == 0);
        self.set_sub(true);
        self.set_half_carry(hc);
        self.set_carry(c);

        self.set_a(result);
        Ok(1)
    }

    fn sbc_a_hl(&mut self, _ins: u8) -> Result<u32> {
        let value = memory::read(self.hl);
        let (result, c, hc) = sub8(self.a(), value, self.carry());

        self.set_zero(result == 0);
        self.set_sub(true);
        self.set_half_carry(hc);
        self.set_carry(c);

        memory::write(self.hl, result);
        Ok(2)
    }

    fn and_a_r8(&mut self, ins: u8) -> Result<u32> {
        let value = self.r8(ins!(ins, r8l))?;
        self.set_a(self.a() & value);

        self.set_zero(self.a() == 0);
        self.set_sub(false);
        self.set_half_carry(true);
        self.set_carry(true);

        Ok(1)
    }

    fn and_a_hl(&mut self, _ins: u8) -> Result<u32> {
        let value = memory::read(self.hl);
        self.set_a(self.a() & value);

        self.set_zero(self.a() == 0);
        self.set_sub(false);
        self.set_half_carry(true);
        self.set_carry(true);

        Ok(2)
    }

    fn xor_a_r8(&mut self, ins: u8) -> Result<u32> {
        let value = self.r8(ins!(ins, r8l))?;
        self.set_a(self.a() ^ value);

        self.set_zero(self.a() == 0);
        self.set_sub(false);
        self.set_half_carry(false);
        self.set_carry(false);

        Ok(1)
    }

    fn xor_a_hl(&mut self, _ins: u8) -> Result<u32> {
        let value = memory::read(self.hl);
        self.set_a(self.a() ^ value);

        self.set_zero(self.a() == 0);
        self.set_sub(false);
        self.set_half_carry(false);
        self.set_carry(false);

        Ok(2)
    }

    fn or_a_r8(&mut self, ins: u8) -> Result<u32> {
        let value = self.r8(ins!(ins, r8l))?;
        self.set_a(self.a() | value);

        self.set_zero(self.a() == 0);
        self.set_sub(false);
        self.set_half_carry(false);
        self.set_carry(false);

        Ok(1)
    }

    fn or_a_hl(&mut self, _ins: u8) -> Result<u32> {
        let value = memory::read(self.hl);
        self.set_a(self.a() | value);

        self.set_zero(self.a() == 0);
        self.set_sub(false);
        self.set_half_carry(false);
        self.set_carry(false);

        Ok(1)
    }

    fn cp_a_r8(&mut self, ins: u8) -> Result<u32> {
        let value = self.r8(ins!(ins, r8l))?;
        let (result, c, hc) = sub8(self.a(), value, false);

        self.set_zero(result == 0);
        self.set_sub(true);
        self.set_half_carry(hc);
        self.set_carry(c);

        Ok(1)
    }

    fn cp_a_hl(&mut self, _ins: u8) -> Result<u32> {
        let value = memory::read(self.hl);
        let (result, c, hc) = sub8(self.a(), value, false);

        self.set_zero(result == 0);
        self.set_sub(true);
        self.set_half_carry(hc);
        self.set_carry(c);

        Ok(2)
    }

    fn add_a_imm8(&mut self, _ins: u8) -> Result<u32> {
        let value = self.imm8();
        let (result, c, hc) = add8(self.a(), value, false);

        self.set_zero(result == 0);
        self.set_sub(false);
        self.set_half_carry(hc);
        self.set_carry(c);

        self.set_a(result);
        Ok(2)
    }

    fn adc_a_imm8(&mut self, _ins: u8) -> Result<u32> {
        let value = self.imm8();
        let (result, c, hc) = add8(self.a(), value, self.carry());

        self.set_zero(result == 0);
        self.set_sub(false);
        self.set_half_carry(hc);
        self.set_carry(c);

        self.set_a(result);
        Ok(2)
    }

    fn sub_a_imm8(&mut self, _ins: u8) -> Result<u32> {
        let value = self.imm8();
        let (result, c, hc) = sub8(self.a(), value, false);

        self.set_zero(result == 0);
        self.set_sub(false);
        self.set_half_carry(hc);
        self.set_carry(c);

        self.set_a(result);
        Ok(2)
    }

    fn sbc_a_imm8(&mut self, _ins: u8) -> Result<u32> {
        let value = self.imm8();
        let (result, c, hc) = sub8(self.a(), value, self.carry());

        self.set_zero(result == 0);
        self.set_sub(false);
        self.set_half_carry(hc);
        self.set_carry(c);

        self.set_a(result);
        Ok(2)
    }

    fn and_a_imm8(&mut self, _ins: u8) -> Result<u32> {
        let value = self.imm8();
        self.set_a(self.a() & value);

        self.set_zero(self.a() == 0);
        self.set_sub(false);
        self.set_half_carry(true);
        self.set_carry(true);

        Ok(2)
    }

    fn xor_a_imm8(&mut self, _ins: u8) -> Result<u32> {
        let value = self.imm8();
        self.set_a(self.a() ^ value);

        self.set_zero(self.a() == 0);
        self.set_sub(false);
        self.set_half_carry(false);
        self.set_carry(false);

        Ok(2)
    }

    fn or_a_imm8(&mut self, _ins: u8) -> Result<u32> {
        let value = self.imm8();
        self.set_a(self.a() | value);

        self.set_zero(self.a() == 0);
        self.set_sub(false);
        self.set_half_carry(false);
        self.set_carry(false);

        Ok(2)
    }

    fn cp_a_imm8(&mut self, _ins: u8) -> Result<u32> {
        let value = self.imm8();
        let (result, c, hc) = sub8(self.a(), value, false);

        self.set_zero(result == 0);
        self.set_sub(true);
        self.set_half_carry(hc);
        self.set_carry(c);

        Ok(2)
    }

    fn ret_cond(&mut self, ins: u8) -> Result<u32> {
        if self.cond(ins!(ins, cond))? {
            self.pc = self.stack_pop();
            Ok(5)
        } else {
            Ok(2)
        }
    }

    fn ret(&mut self, _ins: u8) -> Result<u32> {
        self.pc = self.stack_pop();
        Ok(4)
    }

    fn reti(&mut self, _ins: u8) -> Result<u32> {
        self.pc = self.stack_pop();
        self.ime = true;
        Ok(4)
    }

    fn jp_cond(&mut self, ins: u8) -> Result<u32> {
        if self.cond(ins!(ins, cond))? {
            self.pc = self.imm16();
            Ok(4)
        } else {
            Ok(3)
        }
    }

    fn jp(&mut self, _ins: u8) -> Result<u32> {
        self.pc = self.imm16();
        Ok(4)
    }

    fn jp_hl(&mut self, _ins: u8) -> Result<u32> {
        self.pc = self.hl;
        Ok(1)
    }

    fn call_cond(&mut self, ins: u8) -> Result<u32> {
        if self.cond(ins!(ins, cond))? {
            self.stack_push(self.pc + 2);
            self.pc = self.imm16();
            Ok(6)
        } else {
            Ok(3)
        }
    }

    fn call(&mut self, _ins: u8) -> Result<u32> {
        self.stack_push(self.pc + 2);
        self.pc = self.imm16();
        Ok(6)
    }

    fn rst(&mut self, ins: u8) -> Result<u32> {
        self.stack_push(self.pc + 2);
        self.pc = (ins!(ins, tgt3) as u16) * 8;
        Ok(4)
    }

    fn pop(&mut self, ins: u8) -> Result<u32> {
        let value = self.stack_pop();

        match ins!(ins, r16) {
            0 => self.bc = value,
            1 => self.de = value,
            2 => self.hl = value,
            3 => {
                self.af = value;
                self.set_zero((value & (1 << 7)) != 0);
                self.set_sub((value & (1 << 6)) != 0);
                self.set_half_carry((value & (1 << 5)) != 0);
                self.set_carry((value & (1 << 4)) != 0);
            }
            _ => unreachable!(),
        }

        Ok(3)
    }

    fn push(&mut self, ins: u8) -> Result<u32> {
        let value = match ins!(ins, cond) {
            0 => self.bc,
            1 => self.de,
            2 => self.hl,
            3 => self.af,
            _ => unreachable!(),
        };

        self.stack_push(value);
        Ok(4)
    }

    fn ldh_c_a(&mut self, _ins: u8) -> Result<u32> {
        memory::write(0xff00 | (self.bc & 0xff), (self.af >> 8) as u8);
        Ok(2)
    }

    fn ldh_imm8_a(&mut self, _ins: u8) -> Result<u32> {
        memory::write(0xff00 | (self.imm8() as u16), (self.af >> 8) as u8);
        Ok(3)
    }

    fn ld_imm16_a(&mut self, _ins: u8) -> Result<u32> {
        memory::write(self.imm16(), (self.af >> 8) as u8);
        Ok(4)
    }

    fn ldh_a_c(&mut self, _ins: u8) -> Result<u32> {
        self.af = ((memory::read(0xff00 | (self.bc & 0xff)) as u16) << 8) | (self.af & 0xff);
        Ok(2)
    }

    fn ldh_a_imm8(&mut self, _ins: u8) -> Result<u32> {
        let addr = 0xff00 | (self.imm8() as u16);
        self.set_a(memory::read(addr));
        Ok(3)
    }

    fn ld_a_imm16(&mut self, _ins: u8) -> Result<u32> {
        let value = self.imm16();
        self.set_a(memory::read(value));
        Ok(3)
    }

    fn add_sp_imm8(&mut self, _ins: u8) -> Result<u32> {
        let value = self.imm8();
        let (result, c, hc) = add16(self.sp, value as i8 as i16 as u16, false);

        self.set_zero(false);
        self.set_sub(false);
        self.set_half_carry(hc);
        self.set_carry(c);

        self.sp = result;
        Ok(4)
    }

    fn ld_hl_sp_imm8(&mut self, _ins: u8) -> Result<u32> {
        let value = self.imm8();
        let (result, c, hc) = add16(self.sp, value as i8 as i16 as u16, false);

        self.set_zero(false);
        self.set_sub(false);
        self.set_half_carry(hc);
        self.set_carry(c);

        self.hl = result;
        Ok(3)
    }

    fn ld_sp_hl(&mut self, _ins: u8) -> Result<u32> {
        self.sp = self.hl;
        Ok(2)
    }

    fn di(&mut self, _ins: u8) -> Result<u32> {
        self.ime = false;
        Ok(1)
    }

    fn ei(&mut self, _ins: u8) -> Result<u32> {
        self.ime = true;
        Ok(1)
    }

    fn rlc(&mut self, ins: u8) -> Result<u32> {
        let value = self.r8(ins!(ins, r8l))?.rotate_left(1);
        self.set_r8(ins!(ins, r8l), value)?;

        self.set_zero(value == 0);
        self.set_sub(false);
        self.set_half_carry(false);
        self.set_carry((value & 1) != 0);

        Ok(2)
    }

    fn rlc_hl(&mut self, _ins: u8) -> Result<u32> {
        let value = memory::read(self.hl).rotate_left(1);
        memory::write(self.hl, value);

        self.set_zero(value == 0);
        self.set_sub(false);
        self.set_half_carry(false);
        self.set_carry((value & 1) != 0);

        Ok(4)
    }

    fn rrc(&mut self, ins: u8) -> Result<u32> {
        let value = self.r8(ins!(ins, r8l))?.rotate_right(1);
        self.set_r8(ins!(ins, r8l), value)?;

        self.set_zero(value == 0);
        self.set_sub(false);
        self.set_half_carry(false);
        self.set_carry((value & 0x80) != 0);

        Ok(2)
    }

    fn rrc_hl(&mut self, _ins: u8) -> Result<u32> {
        let value = memory::read(self.hl).rotate_right(1);
        memory::write(self.hl, value);

        self.set_zero(value == 0);
        self.set_sub(false);
        self.set_half_carry(false);
        self.set_carry((value & 0x80) != 0);

        Ok(4)
    }

    fn rl(&mut self, ins: u8) -> Result<u32> {
        let value = self.r8(ins!(ins, r8l))?;
        let bit = if self.carry() { 1 } else { 0 };
        let result = (value << 1) | bit;

        self.set_zero(result == 0);
        self.set_carry((value & 0x80) != 0);
        self.set_sub(false);
        self.set_half_carry(false);

        self.set_r8(ins!(ins, r8l), result)?;
        Ok(2)
    }

    fn rl_hl(&mut self, _ins: u8) -> Result<u32> {
        let value = memory::read(self.hl);
        let bit = if self.carry() { 1 } else { 0 };
        let result = (value << 1) | bit;

        self.set_zero(result == 0);
        self.set_carry((value & 0x80) != 0);
        self.set_sub(false);
        self.set_half_carry(false);

        memory::write(self.hl, result);
        Ok(4)
    }

    fn rr(&mut self, ins: u8) -> Result<u32> {
        let value = self.r8(ins!(ins, r8l))?;
        let bit = if self.carry() { 1 } else { 0 };
        let result = (bit << 7) | (value >> 1);

        self.set_zero(result == 0);
        self.set_carry((value & 0x80) != 0);
        self.set_sub(false);
        self.set_half_carry(false);

        self.set_r8(ins!(ins, r8l), result)?;
        Ok(2)
    }

    fn rr_hl(&mut self, _ins: u8) -> Result<u32> {
        let value = memory::read(self.hl);
        let bit = if self.carry() { 1 } else { 0 };
        let result = (bit << 7) | (value >> 1);

        self.set_zero(result == 0);
        self.set_carry((value & 0x80) != 0);
        self.set_sub(false);
        self.set_half_carry(false);

        memory::write(self.hl, result);
        Ok(4)
    }

    fn sla(&mut self, ins: u8) -> Result<u32> {
        let value = self.r8(ins!(ins, r8l))?;

        self.set_zero((value << 1) != 0);
        self.set_sub(false);
        self.set_half_carry(false);
        self.set_carry((value & 0x80) != 0);

        self.set_r8(ins!(ins, r8l), value << 1)?;
        Ok(2)
    }

    fn sla_hl(&mut self, _ins: u8) -> Result<u32> {
        let value = memory::read(self.hl);

        self.set_zero((value << 1) != 0);
        self.set_sub(false);
        self.set_half_carry(false);
        self.set_carry((value & 0x80) != 0);

        memory::write(self.hl, value << 1);
        Ok(4)
    }

    fn sra(&mut self, ins: u8) -> Result<u32> {
        let value = self.r8(ins!(ins, r8l))?;
        let result = (value & 0x80) | (value >> 1);

        self.set_zero(result != 0);
        self.set_sub(false);
        self.set_half_carry(false);
        self.set_carry((value & 1) != 0);

        self.set_r8(ins!(ins, r8l), result)?;
        Ok(2)
    }

    fn sra_hl(&mut self, _ins: u8) -> Result<u32> {
        let value = memory::read(self.hl);
        let result = (value & 0x80) | (value >> 1);

        self.set_zero(result != 0);
        self.set_sub(false);
        self.set_half_carry(false);
        self.set_carry((value & 1) != 0);

        memory::write(self.hl, result);
        Ok(4)
    }

    fn swap(&mut self, ins: u8) -> Result<u32> {
        let value = self.r8(ins!(ins, r8l))?;
        let result = (value << 4) | (value >> 4);
        self.set_r8(ins!(ins, r8l), result)?;

        self.set_zero(result == 0);
        self.set_sub(false);
        self.set_half_carry(false);
        self.set_carry(false);

        Ok(2)
    }

    fn swap_hl(&mut self, _ins: u8) -> Result<u32> {
        let value = memory::read(self.hl);
        let result = (value << 4) | (value >> 4);
        memory::write(self.hl, result);

        self.set_zero(result == 0);
        self.set_sub(false);
        self.set_half_carry(false);
        self.set_carry(false);

        Ok(4)
    }

    fn srl(&mut self, ins: u8) -> Result<u32> {
        let value = self.r8(ins!(ins, r8l))?;

        self.set_zero((value >> 1) != 0);
        self.set_sub(false);
        self.set_half_carry(false);
        self.set_carry((value & 1) != 0);

        self.set_r8(ins!(ins, r8l), value >> 1)?;
        Ok(2)
    }

    fn srl_hl(&mut self, _ins: u8) -> Result<u32> {
        let value = memory::read(self.hl);

        self.set_zero((value >> 1) != 0);
        self.set_sub(false);
        self.set_half_carry(false);
        self.set_carry((value & 1) != 0);

        memory::write(self.hl, value >> 1);
        Ok(4)
    }

    fn bit(&mut self, ins: u8) -> Result<u32> {
        let value = self.r8(ins!(ins, r8l))?;
        let bit = ins!(ins, b3);

        self.set_zero((value & (1 << bit)) != 0);
        self.set_sub(false);
        self.set_half_carry(true);

        Ok(2)
    }

    fn bit_hl(&mut self, ins: u8) -> Result<u32> {
        let value = memory::read(self.hl);
        let bit = ins!(ins, b3);

        self.set_zero((value & (1 << bit)) != 0);
        self.set_sub(false);
        self.set_half_carry(true);

        Ok(3)
    }

    fn res(&mut self, ins: u8) -> Result<u32> {
        let value = self.r8(ins!(ins, r8l))?;
        let bit = ins!(ins, b3);
        self.set_r8(ins!(ins, r8l), value & !(1 << bit))?;

        Ok(2)
    }

    fn res_hl(&mut self, ins: u8) -> Result<u32> {
        let value = memory::read(self.hl);
        let bit = ins!(ins, b3);
        memory::write(self.hl, value & !(1 << bit));

        Ok(4)
    }

    fn set(&mut self, ins: u8) -> Result<u32> {
        let value = self.r8(ins!(ins, r8l))?;
        let bit = ins!(ins, b3);
        self.set_r8(ins!(ins, r8l), value | (1 << bit))?;

        Ok(2)
    }

    fn set_hl(&mut self, ins: u8) -> Result<u32> {
        let value = memory::read(self.hl);
        let bit = ins!(ins, b3);
        memory::write(self.hl, value | (1 << bit));

        Ok(4)
    }

    fn halt(&mut self, _ins: u8) -> Result<u32> {
        self.halted = true;
        Ok(0)
    }

    fn decode(ins: u8) -> Result<fn(&mut Self, u8) -> Result<u32>> {
        match ins >> 6 {
            0 => match ins {
                0b000000 => Ok(Self::nop),
                0b001000 => Ok(Self::ld_imm16_sp),
                0b000111 => Ok(Self::rlca),
                0b001111 => Ok(Self::rrca),
                0b010111 => Ok(Self::rla),
                0b011111 => Ok(Self::rra),
                0b100111 => Ok(Self::daa),
                0b101111 => Ok(Self::cpl),
                0b110111 => Ok(Self::scf),
                0b111111 => Ok(Self::ccf),
                0b011000 => Ok(Self::jr),
                0b010000 => Ok(Self::stop),
                _ => match ins & 0b1111 {
                    0b0001 => Ok(Self::ld_r16_imm16),
                    0b0010 => Ok(Self::ld_r16mem_a),
                    0b1010 => Ok(Self::ld_a_r16mem),
                    0b0011 => Ok(Self::inc_r16),
                    0b1011 => Ok(Self::dec_r16),
                    0b1001 => Ok(Self::add_hl_r16),
                    _ => match ins & 0b111 {
                        0b100 => Ok(Self::inc_r8),
                        0b101 => Ok(Self::dec_r8),
                        0b110 => Ok(Self::ld_r8_imm8),
                        0b000 if (ins & (1 << 5)) != 0 => Ok(Self::jr_cond),
                        _ => Err(Error::InvalidOperation(ins)),
                    },
                },
            },
            1 => {
                if ins == 0b01110110 {
                    Ok(Self::halt)
                } else {
                    Ok(Self::ld_r8_r8)
                }
            }
            2 => {
                if ins!(ins, r8l) == 6 {
                    match (ins >> 3) & 0b111 {
                        0b000 => Ok(Self::add_a_hl),
                        0b001 => Ok(Self::adc_a_hl),
                        0b010 => Ok(Self::sub_a_hl),
                        0b011 => Ok(Self::sbc_a_hl),
                        0b100 => Ok(Self::and_a_hl),
                        0b101 => Ok(Self::xor_a_hl),
                        0b110 => Ok(Self::or_a_hl),
                        0b111 => Ok(Self::cp_a_hl),
                        _ => unreachable!(),
                    }
                } else {
                    match (ins >> 3) & 0b111 {
                        0b000 => Ok(Self::add_a_r8),
                        0b001 => Ok(Self::adc_a_r8),
                        0b010 => Ok(Self::sub_a_r8),
                        0b011 => Ok(Self::sbc_a_r8),
                        0b100 => Ok(Self::and_a_r8),
                        0b101 => Ok(Self::xor_a_r8),
                        0b110 => Ok(Self::or_a_r8),
                        0b111 => Ok(Self::cp_a_r8),
                        _ => unreachable!(),
                    }
                }
            }
            3 => match ins & 0x3f {
                0b000110 => Ok(Self::add_a_imm8),
                0b001110 => Ok(Self::adc_a_imm8),
                0b010110 => Ok(Self::sub_a_imm8),
                0b011110 => Ok(Self::sbc_a_imm8),
                0b100110 => Ok(Self::and_a_imm8),
                0b101110 => Ok(Self::xor_a_imm8),
                0b110110 => Ok(Self::or_a_imm8),
                0b111110 => Ok(Self::cp_a_imm8),
                0b001001 => Ok(Self::ret),
                0b011001 => Ok(Self::reti),
                0b000011 => Ok(Self::jp),
                0b101001 => Ok(Self::jp_hl),
                0b001101 => Ok(Self::call),
                0b100010 => Ok(Self::ldh_c_a),
                0b100000 => Ok(Self::ldh_imm8_a),
                0b101010 => Ok(Self::ld_imm16_a),
                0b110010 => Ok(Self::ldh_a_c),
                0b110000 => Ok(Self::ldh_a_imm8),
                0b111010 => Ok(Self::ld_a_imm16),
                0b101000 => Ok(Self::add_sp_imm8),
                0b111000 => Ok(Self::ld_hl_sp_imm8),
                0b111001 => Ok(Self::ld_sp_hl),
                0b110011 => Ok(Self::di),
                0b111011 => Ok(Self::ei),
                _ => match ins & 0b1111 {
                    0b0001 => Ok(Self::pop),
                    0b0101 => Ok(Self::push),
                    _ => match ins & 0b111 {
                        0b000 if (ins ^ (1 << 5)) != 0 => Ok(Self::ret_cond),
                        0b010 if (ins ^ (1 << 5)) != 0 => Ok(Self::jp_cond),
                        0b100 if (ins ^ (1 << 5)) != 0 => Ok(Self::call_cond),
                        0b111 => Ok(Self::rst),
                        _ => Err(Error::InvalidOperation(ins)),
                    },
                },
            },
            _ => unreachable!(),
        }
    }

    fn decode_cb(ins: u8) -> Result<fn(&mut Self, u8) -> Result<u32>> {
        match ins >> 6 {
            0b00 => {
                if ins!(ins, r8l) == 6 {
                    match (ins >> 3) & 0b111 {
                        0b000 => Ok(Self::rlc_hl),
                        0b001 => Ok(Self::rrc_hl),
                        0b010 => Ok(Self::rl_hl),
                        0b011 => Ok(Self::rr_hl),
                        0b100 => Ok(Self::sla_hl),
                        0b101 => Ok(Self::sra_hl),
                        0b110 => Ok(Self::swap_hl),
                        0b111 => Ok(Self::srl_hl),
                        _ => unreachable!(),
                    }
                } else {
                    match (ins >> 3) & 0b111 {
                        0b000 => Ok(Self::rlc),
                        0b001 => Ok(Self::rrc),
                        0b010 => Ok(Self::rl),
                        0b011 => Ok(Self::rr),
                        0b100 => Ok(Self::sla),
                        0b101 => Ok(Self::sra),
                        0b110 => Ok(Self::swap),
                        0b111 => Ok(Self::srl),
                        _ => unreachable!(),
                    }
                }
            }
            0b01 => Ok(Self::bit),
            0b10 => Ok(Self::res),
            0b11 => Ok(Self::set),
            _ => unreachable!(),
        }
    }
}
