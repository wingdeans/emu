mod asm;

use crate::asm::{Reg16, Reg8, Rot, AluOp};

#[derive(thiserror::Error, Debug)]
enum TranslateError {
    #[error("IO error")]
    IOErr(#[from] std::io::Error),
}

enum MBC {
    Unk,
    MBC3,
}

static mut MBC: MBC = MBC::Unk;
static mut ROM: Vec<u8> = Vec::new();
static mut BUF: *mut u8 = std::ptr::null_mut();

// Registers
// AF (A)  F  | AX (AL)
// BC (B) (C) | DX (BH) (BL)
// DE (D) (E) | CX (CH) (CL)
// HL (H) (L) | BX (DH) (DL)
// SP         | SI

// fn read16(iter: &mut std::slice::Iter<u8>) -> u16 {
// let lo = *iter.next().unwrap() as u16;
// let hi = *iter.next().unwrap() as u16;
// (hi << 8) | lo
// }

fn read8(iter: &mut std::slice::Iter<u8>) -> u8 {
    *iter.next().unwrap()
}

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

fn translate_insn(iter: &mut std::slice::Iter<u8>) {
    let b = iter.next().unwrap();
    let mut buf = unsafe { BUF };
    match b >> 6 {
        0b00 => match b & 0b111 {
            0b000 => match (b >> 3) & 0b111 {
                0b000 => (),                  // NOP
                0b001 => todo!(),             // LD (u16), SP
                0b010 => todo!(),             // STOP
                0b011 => todo!(),             // JR imm8
                x if x & 0b1 == 1 => todo!(), // JR cond, imm8
                _ => unreachable!(),
            },
            0b001 => {
                if b & 0b1000 == 0 {
                    // LD r16, imm16
                    todo!()
                } else {
                    // ADD HL, r16
                    asm::add_hl_r16(&mut buf, decode_r16((b >> 4) & 0b11))
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
                    todo!()
                } else {
                    // LD A, (r16mem)
                    todo!()
                }
            }
            0b011 => {
                if b & 0b1000 == 0 {
                    // INC r16
                    asm::incdec_r16(&mut buf, decode_r16((b >> 4) & 0b11), false)
                } else {
                    // DEC r16
                    asm::incdec_r16(&mut buf, decode_r16((b >> 4) & 0b11), true)
                }
            }
            // INC r8
            0b100 => asm::incdec_r8(&mut buf, decode_r8((b >> 3) & 0b111), false),
            // DEC r8
            0b101 => asm::incdec_r8(&mut buf, decode_r8((b >> 3) & 0b111), true),
            // LD r8, imm8
            0b110 => asm::mov_r8_imm(&mut buf, decode_r8((b >> 3) & 0b111), read8(iter)),
            0b111 => match (b >> 3) & 0b111 {
                0b000 => asm::rot(&mut buf, Reg8::A, Rot::Rcl), // RLCA
                0b001 => asm::rot(&mut buf, Reg8::A, Rot::Rcr), // RRCA
                0b010 => asm::rot(&mut buf, Reg8::A, Rot::Rol), // RLA
                0b011 => asm::rot(&mut buf, Reg8::A, Rot::Ror), // RRA
                0b100 => todo!(), // DAA
                0b101 => asm::not_a(&mut buf), // CPL
                0b110 => todo!(), // SCF
                0b111 => todo!(), // CCF
                _ => unreachable!(),
            },
            _ => unreachable!(),
        },
        // LD r8, r8
        0b01 => asm::mov_r8_r8(&mut buf, decode_r8((b >> 3) & 0b111), decode_r8(b & 0b111)),
        0b10 => match (b >> 3) & 0b111 {
            // ADD A, r8
            0b000 => asm::alu_a_r8(&mut buf, decode_r8(b & 0b111), AluOp::Add),
            // ADC A, r8
            0b001 => asm::alu_a_r8(&mut buf, decode_r8(b & 0b111), AluOp::Adc),
            // SUB A, r8
            0b010 => asm::alu_a_r8(&mut buf, decode_r8(b & 0b111), AluOp::Sub),
            // SBC A, r8
            0b011 => asm::alu_a_r8(&mut buf, decode_r8(b & 0b111), AluOp::Sbb),
            // AND A, r8
            0b100 => asm::alu_a_r8(&mut buf, decode_r8(b & 0b111), AluOp::And),
            // XOR A, r8
            0b101 => asm::alu_a_r8(&mut buf, decode_r8(b & 0b111), AluOp::Xor),
            // OR A, r8
            0b110 => asm::alu_a_r8(&mut buf, decode_r8(b & 0b111), AluOp::Or),
            // CP A, r8
            0b111 => asm::alu_a_r8(&mut buf, decode_r8(b & 0b111), AluOp::Cmp),
            _ => unreachable!(),
        },
        0b11 => match b & 0b111 {
            0b000 => match (b >> 3) & 0b111 {
                0b000..=0b011 => todo!(), // RET cond,
                0b100 => todo!(),         // LDH (imm8), A
                0b101 => todo!(),         // ADD SP, imm8
                0b110 => todo!(),         // LDH A, (imm8)
                // LD HL, SP + imm8
                0b111 => asm::mov_hl_sp_imm(&mut buf, read8(iter)),
                _ => unreachable!(),
            },
            0b001 => match (b >> 3) & 0b111 {
                x if x & 0b1 == 0 => (), // POP r16stk
                0b001 => (),             // RET
                0b011 => (),             // RETI
                0b101 => (),             // JP HL
                // LD SP, HL
                0b111 => asm::mov_r16_r16(&mut buf, Reg16::SP, Reg16::HL),
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
                    let b = iter.next().unwrap();
                    match (b >> 6) & 0b11 {
                        0b00 => match (b >> 2) & 0b111 {
                            // RLC r8
                            0b000 => asm::rot(&mut buf, decode_r8(b & 0b111), Rot::Rcl),
                            // RRC r8
                            0b001 => asm::rot(&mut buf, decode_r8(b & 0b111), Rot::Rcr),
                            // RL r8
                            0b010 => asm::rot(&mut buf, decode_r8(b & 0b111), Rot::Rol),
                            // RR r8
                            0b011 => asm::rot(&mut buf, decode_r8(b & 0b111), Rot::Ror),
                            // SLA r8
                            0b100 => asm::rot(&mut buf, decode_r8(b & 0b111), Rot::Shl),
                            // SRA r8
                            0b101 => asm::rot(&mut buf, decode_r8(b & 0b111), Rot::Sar),
                            // SWAP r8
                            0b110 => asm::rot4(&mut buf, decode_r8(b & 0b111)),
                            // SRL r8
                            0b111 => asm::rot(&mut buf, decode_r8(b & 0b111), Rot::Shr),
                            _ => unreachable!()
                        }
                        // BIT b3, r8
                        0b01 => (),
                        // RES b3, r8
                        0b10 => (),
                        // SET b3, r8
                        0b11 => (),
                        _ => unreachable!()
                    }
                }
                0b010..0b110 => unreachable!("invalid opcode"),
                0b110 => (), // DI
                0b111 => (), // EI
                _ => unreachable!(),
            },
            0b100 => {
                if b & 0b10_000 == 0 {
                    // CALL cond, imm16
                } else {
                    unreachable!("invalid opcode")
                }
            }
            0b101 => match (b >> 3) & 0b111 {
                x if x & 0b1 == 0 => (), // PUSH r16stk
                0b001 => (),             // CALL imm16
                _ => unreachable!("invalid opcode"),
            },
            0b110 => match (b >> 3) & 0b111 {
                // ADD A, imm8
                0b000 => asm::alu_a_imm(&mut buf, read8(iter), AluOp::Add),
                // ADC A, imm8
                0b001 => asm::alu_a_imm(&mut buf, read8(iter), AluOp::Adc),
                // SUB A, imm8
                0b010 => asm::alu_a_imm(&mut buf, read8(iter), AluOp::Sub),
                // SBC A, imm8
                0b011 => asm::alu_a_imm(&mut buf, read8(iter), AluOp::Sbb),
                // AND A, imm8
                0b100 => asm::alu_a_imm(&mut buf, read8(iter), AluOp::And),
                // XOR A, imm8
                0b101 => asm::alu_a_imm(&mut buf, read8(iter), AluOp::Xor),
                // OR  A, imm8
                0b110 => asm::alu_a_imm(&mut buf, read8(iter), AluOp::Or),
                // CP  A, imm8
                0b111 => asm::alu_a_imm(&mut buf, read8(iter), AluOp::Cmp),
                _ => unreachable!(),
            },
            0b111 => (), // RST tgt3
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
    unsafe { BUF = buf };
}

fn translate_bb(pc: u16, bank: u8) {
    let mut iter = unsafe { ROM[pc as usize..].iter() };
    translate_insn(&mut iter);
}

fn main() -> Result<(), TranslateError> {
    let args: Vec<_> = std::env::args().collect();

    unsafe {
        BUF = libc::mmap(
            std::ptr::null_mut(),
            0x100_0000,
            libc::PROT_WRITE | libc::PROT_EXEC,
            libc::MAP_ANONYMOUS,
            -1,
            0,
        ) as *mut u8;
    };

    let buf = std::fs::read(&args[1])?;
    let mbc = match buf[0x147] {
        0x0F..=0x13 => MBC::MBC3,
        unk => todo!("mapper 0x{:x}", unk),
    };
    unsafe {
        ROM = buf;
        MBC = mbc;
    };

    translate_bb(0x100, u8::MAX);

    Ok(())
}
