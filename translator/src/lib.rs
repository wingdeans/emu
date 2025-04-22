mod asm;

#[cfg(feature = "gbit")]
mod gbit;

use crate::asm::{AluOp, Reg16, Reg8, Rot};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(thiserror::Error, Debug)]
enum TranslateError {
    #[error("IO error")]
    IOErr(#[from] std::io::Error),
}

#[derive(Default)]
#[repr(C)]
struct Regs {
    // AF (AL) (AH)
    a: u8,
    f: u8,
    // BC
    c: u8,
    b: u8,
    // DE
    e: u8,
    d: u8,
    // HL
    l: u8,
    h: u8,
    // SP
    sp: u16,
}

struct Cpu<T: library::bus::Addressable> {
    adrb: Rc<RefCell<T>>,
    buf: *mut u8,
    pc: u16,
    regs: Regs,
}

// Registers
// AF (A)  F  | AX (AL) [AH, DIL=N, R9L=H] // flipped
// BC (B) (C) | BX (BH) (BL)
// DE (D) (E) | CX (CH) (CL)
// HL (H) (L) | DX (DH) (DL)
// SP         | SI

// fn read16(iter: &mut std::slice::Iter<u8>) -> u16 {
// let lo = *iter.next().unwrap() as u16;
// let hi = *iter.next().unwrap() as u16;
// (hi << 8) | lo
// }

fn decode_r16(b: u8) -> Reg16 {
    match b {
        0 => Reg16::BC,
        1 => Reg16::DE,
        2 => Reg16::HL,
        3 => Reg16::SP,
        _ => unreachable!(),
    }
}

fn decode_r8(b: u8) -> Reg8 {
    match b {
        0 => Reg8::B,
        1 => Reg8::C,
        2 => Reg8::D,
        3 => Reg8::E,
        4 => Reg8::H,
        5 => Reg8::L,
        6 => Reg8::MemHL,
        7 => Reg8::A,
        _ => unreachable!(),
    }
}

impl<T: library::bus::Addressable> Cpu<T> {
    fn new(adrb: Rc<RefCell<T>>) -> Self {
        let buf = unsafe {
            libc::mmap(
                std::ptr::null_mut(),
                0x1_0000_0000,
                libc::PROT_WRITE | libc::PROT_EXEC,
                libc::MAP_ANONYMOUS | libc::MAP_PRIVATE,
                -1,
                0,
            ) as *mut u8
        };

        Self {
            adrb,
            buf,
            pc: 0,
            regs: Regs::default(),
        }
    }

    fn read8(&mut self) -> u8 {
        let v = self.adrb.borrow_mut().read(self.pc).unwrap();
        self.pc += 1;
        v
    }

    fn translate_insn(&mut self) -> Result<(), ()> {
        let b = self.read8();
        match b >> 6 {
            0b00 => match b & 0b111 {
                0b000 => match (b >> 3) & 0b111 {
                    0b000 => (),                             // NOP
                    0b001 => println!("TODO"),               // LD (u16), SP
                    0b010 => return Err(()),                 // STOP
                    0b011 => println!("TODO"),               // JR imm8
                    x if x & 0b100 != 0 => println!("TODO"), // JR cond, imm8
                    _ => unreachable!(),
                },
                0b001 => {
                    if b & 0b1000 == 0 {
                        // LD r16, imm16
                        println!("TODO")
                    } else {
                        // ADD HL, r16
                        asm::add_hl_r16(&mut self.buf, decode_r16((b >> 4) & 0b11));
                    }
                }
                0b010 => {
                    // let (reg, inc) = match (b >> 4) & 0b111 {
                    // 0 => (Reg16::BC, 0),
                    // 1 => (Reg16::DE, 0),
                    // 2 => (Reg16::HL, 1),
                    // 3 => (Reg16::HL, -1),
                    // _ => unreachable!(),
                    // };
                    if b & 0b1000 == 0 {
                        // LD (r16mem), A
                        println!("TODO")
                    } else {
                        // LD A, (r16mem)
                        println!("TODO")
                    }
                }
                0b011 => {
                    if b & 0b1000 == 0 {
                        // INC r16
                        asm::incdec_r16(&mut self.buf, decode_r16((b >> 4) & 0b11), false);
                    } else {
                        // DEC r16
                        asm::incdec_r16(&mut self.buf, decode_r16((b >> 4) & 0b11), true);
                    }
                }
                // INC r8
                0b100 => asm::incdec_r8(&mut self.buf, decode_r8((b >> 3) & 0b111), false),
                // DEC r8
                0b101 => asm::incdec_r8(&mut self.buf, decode_r8((b >> 3) & 0b111), true),
                // LD r8, imm8
                0b110 => {
                    let imm = self.read8();
                    asm::mov_r8_imm(&mut self.buf, decode_r8((b >> 3) & 0b111), imm)
                }
                0b111 => match (b >> 3) & 0b111 {
                    0b000 => {
                        // RLCA
                        asm::rot(&mut self.buf, Reg8::A, Rot::Rol);
                        asm::clear_n(&mut self.buf);
                        asm::clear_h(&mut self.buf)
                    }
                    0b001 => {
                        // RRCA
                        asm::rot(&mut self.buf, Reg8::A, Rot::Ror);
                        asm::clear_n(&mut self.buf);
                        asm::clear_h(&mut self.buf)
                    }
                    0b010 => asm::rot(&mut self.buf, Reg8::A, Rot::Rcl), // RLA
                    0b011 => asm::rot(&mut self.buf, Reg8::A, Rot::Rcr), // RRA
                    0b100 => println!("TODO"),                           // DAA
                    0b101 => asm::not_a(&mut self.buf),                  // CPL
                    0b110 => println!("TODO"),                           // SCF
                    0b111 => println!("TODO"),                           // CCF
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            },
            // LD r8, r8
            0b01 => asm::mov_r8_r8(
                &mut self.buf,
                decode_r8((b >> 3) & 0b111),
                decode_r8(b & 0b111),
            ),
            0b10 => {
                let reg = decode_r8(b & 0b111);
                match (b >> 3) & 0b111 {
                    // ADD A, r8
                    0b000 => {
                        asm::alu_a_r8(&mut self.buf, reg, AluOp::Add);
                        asm::clear_n(&mut self.buf);
                        asm::save_hc(&mut self.buf);
                    }
                    // ADC A, r8
                    0b001 => {
                        asm::alu_a_r8(&mut self.buf, reg, AluOp::Adc);
                        asm::clear_n(&mut self.buf);
                        asm::save_hc(&mut self.buf);
                    }
                    // SUB A, r8
                    0b010 => {
                        asm::alu_a_r8(&mut self.buf, reg, AluOp::Sub);
                        asm::set_n(&mut self.buf);
                        asm::save_hc(&mut self.buf);
                    }
                    // SBC A, r8
                    0b011 => {
                        asm::alu_a_r8(&mut self.buf, reg, AluOp::Sbb);
                        asm::set_n(&mut self.buf);
                        asm::save_hc(&mut self.buf);
                    }
                    // AND A, r8
                    0b100 => {
                        asm::alu_a_r8(&mut self.buf, reg, AluOp::And);
                        asm::clear_n(&mut self.buf);
                        asm::set_h(&mut self.buf);
                    }
                    // XOR A, r8
                    0b101 => {
                        asm::alu_a_r8(&mut self.buf, reg, AluOp::Xor);
                        asm::clear_n(&mut self.buf);
                        asm::clear_h(&mut self.buf);
                    }
                    // OR A, r8
                    0b110 => {
                        asm::alu_a_r8(&mut self.buf, reg, AluOp::Or);
                        asm::clear_n(&mut self.buf);
                        asm::clear_h(&mut self.buf);
                    }
                    // CP A, r8
                    0b111 => asm::alu_a_r8(&mut self.buf, reg, AluOp::Cmp),
                    _ => unreachable!(),
                }
            }
            0b11 => match b & 0b111 {
                0b000 => match (b >> 3) & 0b111 {
                    0b000..=0b011 => println!("TODO"), // RET cond,
                    0b100 => println!("TODO"),         // LDH (imm8), A
                    0b101 => println!("TODO"),         // ADD SP, imm8
                    0b110 => println!("TODO"),         // LDH A, (imm8)
                    // LD HL, SP + imm8
                    0b111 => {
                        let imm = self.read8();
                        asm::mov_hl_sp_imm(&mut self.buf, imm)
                    }
                    _ => unreachable!(),
                },
                0b001 => match (b >> 3) & 0b111 {
                    x if x & 0b1 == 0 => (), // POP r16stk
                    0b001 => (),             // RET
                    0b011 => (),             // RETI
                    0b101 => (),             // JP HL
                    // LD SP, HL
                    0b111 => asm::mov_r16_r16(&mut self.buf, Reg16::SP, Reg16::HL),
                    _ => unreachable!(),
                },
                0b010 => match (b >> 3) & 0b111 {
                    0b000..=0b011 => (), // JP cond, imm16
                    0b100 => (),         // LDH (C), A
                    0b101 => (),         // LDH (imm16), A
                    0b110 => (),         // LDH A, (C)
                    0b111 => (),         // LDH A, (imm16)
                    _ => unreachable!(),
                },
                0b011 => match (b >> 3) & 0b111 {
                    0b000 => (), // JP imm16
                    // PREFIX
                    0b001 => {
                        let b = self.read8();
                        let b3 = (b >> 3) & 0b111;
                        let reg = decode_r8(b & 0b111);
                        match (b >> 6) & 0b11 {
                            0b00 => {
                                match b3 {
                                    // RLC r8
                                    0b000 => {
                                        asm::rot(&mut self.buf, reg, Rot::Rol);
                                        asm::clear_n(&mut self.buf);
                                        asm::clear_h(&mut self.buf);
                                        asm::test_r8_zero(&mut self.buf, reg);
                                    }
                                    // RRC r8
                                    0b001 => {
                                        asm::rot(&mut self.buf, reg, Rot::Ror);
                                        asm::clear_n(&mut self.buf);
                                        asm::clear_h(&mut self.buf);
                                        asm::test_r8_zero(&mut self.buf, reg);
                                    }
                                    // RL r8
                                    0b010 => {
                                        asm::rot(&mut self.buf, reg, Rot::Rcl);
                                        asm::clear_n(&mut self.buf);
                                        asm::clear_h(&mut self.buf);
                                        asm::test_r8_zero(&mut self.buf, reg);
                                    }
                                    // RR r8
                                    0b011 => {
                                        asm::rot(&mut self.buf, reg, Rot::Rcr);
                                        asm::clear_n(&mut self.buf);
                                        asm::clear_h(&mut self.buf);
                                        asm::test_r8_zero(&mut self.buf, reg);
                                    }
                                    // SLA r8
                                    0b100 => {
                                        asm::rot(&mut self.buf, reg, Rot::Shl);
                                        asm::clear_n(&mut self.buf);
                                        asm::clear_h(&mut self.buf);
                                    }
                                    // SRA r8
                                    0b101 => {
                                        asm::rot(&mut self.buf, reg, Rot::Sar);
                                        asm::clear_n(&mut self.buf);
                                        asm::clear_h(&mut self.buf);
                                    }
                                    // SWAP r8
                                    0b110 => {
                                        asm::rot4(&mut self.buf, reg);
                                        asm::clear_n(&mut self.buf);
                                        asm::clear_h(&mut self.buf);
                                        asm::test_r8_zero_raw(&mut self.buf, reg);
                                    }
                                    // SRL r8
                                    0b111 => {
                                        asm::rot(&mut self.buf, reg, Rot::Shr);
                                        asm::clear_n(&mut self.buf);
                                        asm::clear_h(&mut self.buf);
                                    }
                                    _ => unreachable!(),
                                }
                            }
                            // BIT b3, r8
                            0b01 => {
                                asm::test_r8_imm(&mut self.buf, reg, 1 << b3);
                                asm::clear_n(&mut self.buf);
                                asm::set_h(&mut self.buf);
                            }
                            // RES b3, r8
                            0b10 => asm::and_flagless(&mut self.buf, reg, !(1 << b3)),
                            // SET b3, r8
                            0b11 => asm::or_flagless(&mut self.buf, reg, 1 << b3),
                            _ => unreachable!(),
                        }
                    }
                    0b010..0b110 => return Err(()), // invalid opcode
                    0b110 => (),                    // DI
                    0b111 => (),                    // EI
                    _ => unreachable!(),
                },
                0b100 => {
                    if b & 0b10_000 == 0 {
                        // CALL cond, imm16
                    } else {
                        return Err(()); // invalid opcode
                    }
                }
                0b101 => match (b >> 3) & 0b111 {
                    x if x & 0b1 == 0 => (), // PUSH r16stk
                    0b001 => (),             // CALL imm16
                    _ => return Err(()),     // invalid opcode
                },
                0b110 => {
                    let imm = self.read8();
                    match (b >> 3) & 0b111 {
                        // ADD A, imm8
                        0b000 => asm::alu_a_imm(&mut self.buf, imm, AluOp::Add),
                        // ADC A, imm8
                        0b001 => asm::alu_a_imm(&mut self.buf, imm, AluOp::Adc),
                        // SUB A, imm8
                        0b010 => asm::alu_a_imm(&mut self.buf, imm, AluOp::Sub),
                        // SBC A, imm8
                        0b011 => asm::alu_a_imm(&mut self.buf, imm, AluOp::Sbb),
                        // AND A, imm8
                        0b100 => {
                            asm::alu_a_imm(&mut self.buf, imm, AluOp::And);
                            asm::clear_n(&mut self.buf);
                            asm::set_h(&mut self.buf);
                        }
                        // XOR A, imm8
                        0b101 => {
                            asm::alu_a_imm(&mut self.buf, imm, AluOp::Xor);
                            asm::clear_n(&mut self.buf);
                            asm::clear_h(&mut self.buf);
                        }
                        // OR  A, imm8
                        0b110 => {
                            asm::alu_a_imm(&mut self.buf, imm, AluOp::Or);
                            asm::clear_n(&mut self.buf);
                            asm::clear_h(&mut self.buf);
                        }
                        // CP  A, imm8
                        0b111 => asm::alu_a_imm(&mut self.buf, imm, AluOp::Cmp),
                        _ => unreachable!(),
                    }
                }
                0b111 => (), // RST tgt3
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
        Ok(())
    }
}

impl<T: library::bus::Addressable> library::cpu::Cpu for Cpu<T> {
    fn execute(&mut self) -> u32 {
        let buf = self.buf;
        asm::load_regs(&mut self.buf, &self.regs);
        let _ = self.translate_insn(); // TODO
        asm::save_regs(&mut self.buf, &self.regs);
        asm::ret(&mut self.buf);
        let func: fn() = unsafe { std::mem::transmute(buf) };
        func();
        0
    }
    fn ime(&self) -> bool {
        todo!()
    }
    fn int(&mut self, _: u16) {
        todo!()
    }
}
