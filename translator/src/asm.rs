#[derive(Clone, Copy)]
pub(crate) enum Reg16 {
    AF = 0,
    BC = 3,
    DE = 1,
    HL = 2,
    SP = 6,
}

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum Reg8 {
    A = 0,
    F = 4,
    B = 7,
    C = 3,
    D = 5,
    E = 1,
    H = 6,
    L = 2,
    MemHL = 0xFF,
}

#[derive(Clone, Copy)]
pub(crate) enum Rot {
    Rol = 0,
    Ror = 1,
    Rcl = 2,
    Rcr = 3,
    Shl = 4,
    Sar = 7,
    Shr = 5,
}

#[derive(Clone, Copy)]
pub(crate) enum AluOp {
    Add = 0x00,
    Adc = 0x10,
    Sub = 0x28,
    Sbb = 0x18,
    And = 0x20,
    Xor = 0x30,
    Or = 0x08,
    Cmp = 0x38,
}

fn write(dst: &mut *mut u8, src: &[u8]) {
    unsafe {
        std::ptr::copy(src.as_ptr(), *dst, src.len());
        *dst = dst.add(src.len());
    }
}

pub(crate) fn ret(buf: &mut *mut u8) {
    write(buf, &[0xC3])
}

fn movabs_r08(buf: &mut *mut u8, addr: *const libc::c_void) {
    let addr = addr as u64;
    write(
        buf,
        &[
            0x49, // prefix
            0xB8, // MOV r64, imm64
            addr as u8,
            (addr >> 8) as u8,
            (addr >> 16) as u8,
            (addr >> 24) as u8,
            (addr >> 32) as u8,
            (addr >> 40) as u8,
            (addr >> 48) as u8,
            (addr >> 56) as u8,
        ],
    )
}

fn mov_r08addr_r16(buf: &mut *mut u8, reg: Reg16) {
    write(
        buf,
        &[
            0x66, // prefix
            0x41,
            0x89, // MOV r/m16, r16
            0b00_000_000 | ((reg as u8) << 3),
        ],
    )
}

fn mov_r16_r08addr(buf: &mut *mut u8, reg: Reg16) {
    write(
        buf,
        &[
            0x66, // prefix
            0x41,
            0x8B, // MOV r16, r/m16
            0b00_000_000 | ((reg as u8) << 3),
        ],
    )
}

fn lea_r08_add_imm8(buf: &mut *mut u8, imm: i8) {
    write(
        buf,
        &[
            0x4D, // prefix
            0x8D, // LEA r64,m
            0b01_000_000,
            imm as u8,
        ],
    )
}

fn rot_r8_imm8(buf: &mut *mut u8, reg: Reg8, imm: u8) {
    write(
        buf,
        &[
            0xC0, // SAL r/m8, imm8
            0b11_000_000 | (4 << 3) | reg as u8,
            imm,
        ],
    )
}

fn or_r8_r8(buf: &mut *mut u8, dst: Reg8, src: Reg8) {
    write(
        buf,
        &[
            0x08, // OR r/m8, r8
            0b11_000_000 | ((src as u8) << 3) | dst as u8,
        ],
    )
}

fn and_r8_imm(buf: &mut *mut u8, reg: Reg8, imm: u8) {
    write(
        buf,
        &[
            0x80, // AND r/m8, imm8
            0b11_000_000 | (4 << 3) | reg as u8,
            imm,
        ],
    )
}

pub(crate) fn test_r8_zero_raw(buf: &mut *mut u8, reg: Reg8) {
    write(
        buf,
        &[
            // TEST r/m8, r8
            0x84,
            0b11_000_000 | ((reg as u8) << 3) | reg as u8,
        ],
    )
}

pub(crate) fn test_r8_zero(buf: &mut *mut u8, reg: Reg8) {
    write(
        buf,
        &[
            // LAHF
            0x9F,
            // and ah, 0b1011_1111
            0x80,
            0xe4,
            0b1011_1111,
            // TEST r/m8, r8
            0x84,
            0b11_000_000 | ((reg as u8) << 3) | reg as u8,
            // jne 3
            0x75,
            0x03,
            0x80,
            0xcc,
            0b0100_0000,
            // SAHF
            0x9E,
        ],
    )
}

pub(crate) fn test_r8_imm(buf: &mut *mut u8, reg: Reg8, imm: u8) {
    if reg != Reg8::MemHL {
        write(
            buf,
            &[
                // LAHF
                0x9F,
                // and ah, 0b1011_1111
                0x80,
                0xe4,
                0b1011_1111,
                // TEST r/m8, r8
                0xF6,
                0b11_000_000 | reg as u8,
                imm,
                // jne 3
                0x75,
                0x03,
                0x80,
                0xcc,
                0b0100_0000,
                // SAHF
                0x9E,
            ],
        )
    }
}

pub(crate) fn save_hc(buf: &mut *mut u8) {
    write(
        buf,
        &[
            // LAHF
            0x9F,
            // TEST r/m8, 0b1_0000
            0xF6,
            0b11_000_000 | Reg8::F as u8,
            0b1_0000,
            // SETNZ r9l
            0x41, 0x0f, 0x95, 0xc1,
            // shl r9b, 5
            0x41, 0xc0, 0xe1, 0x05,
            // SAHF
            0x9E,
        ],
    )
}

pub(crate) fn load_regs(buf: &mut *mut u8, regs: &crate::Regs) {
    write(buf, &[0x53]); // push rbx
    movabs_r08(buf, (regs as *const crate::Regs) as *const libc::c_void);
    mov_r16_r08addr(buf, Reg16::AF);

    write(buf, &[0x80, 0xf4, 0b1000_0000]); // XOR: invert F.Z
    write(buf, &[0x0f, 0xb6, 0xcc]); // MOVZX ECX, AH

    write(buf, &[0x40, 0x88, 0xcf]); // MOV DIL, CL
    write(buf, &[0x40, 0x80, 0xe7, 0b0100_0000]); // and DIL
    write(buf, &[0x41, 0x88, 0xc9]); // MOV R9L, CL
    write(buf, &[0x41, 0x80, 0xe1, 0b0010_0000]); // and R9L

    write(buf, &[0x83, 0xe1, 0b0001_0000]); // and ecx

    write(buf, &[0xf6, 0xc4, 0b1000_0000]); // TEST: set ZF

    write(buf, &[0x67, 0xE3, 0x01]); // jecxz
    {
        write(buf, &[0xF9]); // STC
    }

    lea_r08_add_imm8(buf, 2);
    mov_r16_r08addr(buf, Reg16::BC);
    lea_r08_add_imm8(buf, 2);
    mov_r16_r08addr(buf, Reg16::DE);
    lea_r08_add_imm8(buf, 2);
    mov_r16_r08addr(buf, Reg16::HL);
    lea_r08_add_imm8(buf, 2);
    mov_r16_r08addr(buf, Reg16::SP);
}

pub(crate) fn save_regs(buf: &mut *mut u8, regs: &crate::Regs) {
    movabs_r08(
        buf,
        ((regs as *const crate::Regs) as *const libc::c_void).wrapping_add(2),
    );
    mov_r08addr_r16(buf, Reg16::BC);
    lea_r08_add_imm8(buf, -2);

    // save flags after BH, BL are free
    write(buf, &[0x0f, 0x92, 0b11_000_000 | Reg8::B as u8]); // SETC r/m8
    write(buf, &[0x0f, 0x94, 0b11_000_000 | Reg8::C as u8]); // SETE r/m8
    rot_r8_imm8(buf, Reg8::B, 4);
    rot_r8_imm8(buf, Reg8::C, 7);
    or_r8_r8(buf, Reg8::C, Reg8::B);
    write(buf, &[0x40, 0x08, 0xfb]); // OR BL, DIL
    write(buf, &[0x44, 0x08, 0xCB]); // OR BL, R9L
    mov_r8_r8(buf, Reg8::F, Reg8::C);
    mov_r08addr_r16(buf, Reg16::AF);

    lea_r08_add_imm8(buf, 4);
    mov_r08addr_r16(buf, Reg16::DE);
    lea_r08_add_imm8(buf, 2);
    mov_r08addr_r16(buf, Reg16::HL);
    lea_r08_add_imm8(buf, 2);
    mov_r08addr_r16(buf, Reg16::SP);
    write(buf, &[0x5B]); // pop rbx
}

pub(crate) fn clear_n(buf: &mut *mut u8) {
    // mov dil, 0
    write(buf, &[0x40, 0xb7, 0x00])
}

pub(crate) fn clear_h(buf: &mut *mut u8) {
    // mov r9l, 0
    write(buf, &[0x41, 0xb1, 0x00])
}

pub(crate) fn set_n(buf: &mut *mut u8) {
    // mov dil, 0
    write(buf, &[0x40, 0xb7, 0b0100_0000])
}

pub(crate) fn set_h(buf: &mut *mut u8) {
    // mov r9l, 0
    write(buf, &[0x41, 0xb1, 0b0010_0000])
}

pub(crate) fn add_hl_r16(buf: &mut *mut u8, reg: Reg16) {
    write(
        buf,
        &[
            0x66, // prefix
            0x01, // ADD r/m16, r16
            0b11_000_000 | ((reg as u8) << 3) | Reg16::HL as u8,
        ],
    )
}

pub(crate) fn incdec_r16(buf: &mut *mut u8, reg: Reg16, dec: bool) {
    write(
        buf,
        &[
            0x66, // prefix
            0x8D, // LEA r16,m
            0b01_000_000 | ((reg as u8) << 3) | reg as u8,
            if dec { 0xFF } else { 1 },
        ],
    )
}

pub(crate) fn incdec_r8(buf: &mut *mut u8, reg: Reg8, dec: bool) {
    if reg != Reg8::MemHL {
        write(
            buf,
            &[
                0xFE, // INC r/m8
                0b11_000_000 | reg as u8 | ((dec as u8) << 3),
            ],
        )
    }
}

pub(crate) fn mov_r8_imm(buf: &mut *mut u8, reg: Reg8, imm: u8) {
    if reg != Reg8::MemHL {
        write(
            buf,
            &[
                0xB0 + reg as u8, // MOV r8, imm8
                imm,
            ],
        )
    }
}

pub(crate) fn rot(buf: &mut *mut u8, reg: Reg8, rot: Rot) {
    write(
        buf,
        &[
            0xD0, // r/m8, 1
            0b11_000_000 | ((rot as u8) << 3) | reg as u8,
        ],
    )
}

pub(crate) fn not_a(buf: &mut *mut u8) {
    write(
        buf,
        &[
            0xF6, // NOT r/m8
            0b11_010_000 | Reg8::A as u8,
        ],
    )
}

pub(crate) fn mov_r8_r8(buf: &mut *mut u8, dst: Reg8, src: Reg8) {
    if dst != Reg8::MemHL && src != Reg8::MemHL {
        write(
            buf,
            &[
                0x88, // MOV r/m8, r8
                0b11_000_000 | ((src as u8) << 3) | dst as u8,
            ],
        )
    }
}

pub(crate) fn alu_a_r8(buf: &mut *mut u8, src: Reg8, op: AluOp) {
    if src != Reg8::MemHL {
        write(
            buf,
            &[
                op as u8, // r/m8, r8
                0b11_000_000 | ((src as u8) << 3) | Reg8::A as u8,
            ],
        )
    }
}

pub(crate) fn mov_r16_r16(buf: &mut *mut u8, dst: Reg16, src: Reg16) {
    write(
        buf,
        &[
            0x66, // prefix
            0x89, // MOV r/m16, r16
            0b11_000_000 | ((src as u8) << 3) | dst as u8,
        ],
    )
}

pub(crate) fn mov_hl_sp_imm(buf: &mut *mut u8, imm: u8) {
    mov_r16_r16(buf, Reg16::HL, Reg16::SP);
    write(
        buf,
        &[
            0x66, // prefix
            0x83, // ADD r/m16, imm8
            0b11_000_000 | Reg16::HL as u8,
            imm,
        ],
    );
}

pub(crate) fn rot4(buf: &mut *mut u8, reg: Reg8) {
    write(
        buf,
        &[
            0xC0, // ROL r/m8, imm8
            0b11_000_000 | reg as u8,
            4,
        ],
    )
}

pub(crate) fn alu_a_imm(buf: &mut *mut u8, imm: u8, op: AluOp) {
    write(
        buf,
        &[
            4 + op as u8, // r/m8, r8
            imm,
        ],
    )
}

pub(crate) fn or_flagless(buf: &mut *mut u8, reg: Reg8, imm: u8) {
    if reg != Reg8::MemHL {
        write(
            buf,
            &[
                // LAHF
                0x9F,
                // OR r/m8, imm8
                0x80,
                0b11_001_000 | reg as u8,
                imm,
                // SAHF
                0x9E,
            ],
        )
    }
}

pub(crate) fn and_flagless(buf: &mut *mut u8, reg: Reg8, imm: u8) {
    if reg != Reg8::MemHL {
        write(
            buf,
            &[
                // LAHF
                0x9F,
                // AND r/m8, imm8
                0x80,
                0b11_100_000 | reg as u8,
                imm,
                // SAHF
                0x9E,
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Decoder<'a>(iced_x86::Decoder<'a>, iced_x86::IntelFormatter);

    impl<'a> Decoder<'a> {
        fn new(buf: &'a [u8]) -> Self {
            Decoder(
                iced_x86::Decoder::new(64, buf, iced_x86::DecoderOptions::NONE),
                iced_x86::IntelFormatter::new(),
            )
        }

        fn decode(&mut self) -> String {
            use iced_x86::Formatter;
            let mut output = String::new();
            let instr = self.0.decode();
            self.1.format(&instr, &mut output);
            output
        }
    }

    fn decode(buf: &[u8]) -> String {
        let mut decoder = Decoder::new(buf);
        decoder.decode()
    }

    #[test]
    fn test_add_hl_r16() {
        let mut buf = [0; 64];
        add_hl_r16(&mut buf.as_mut_ptr(), Reg16::BC);
        assert_eq!(decode(&buf), "add dx,bx");
    }

    #[test]
    fn test_incdec_r16() {
        let mut buf = [0; 64];
        incdec_r16(&mut buf.as_mut_ptr(), Reg16::BC, false);
        assert_eq!(decode(&buf), "lea bx,[rbx+1]");
        incdec_r16(&mut buf.as_mut_ptr(), Reg16::BC, true);
        assert_eq!(decode(&buf), "lea bx,[rbx-1]");

        clear_n(&mut buf.as_mut_ptr());
        assert_eq!(decode(&buf), "mov dil,0");
    }

    #[test]
    fn test_incdec_r8() {
        let mut buf = [0; 64];
        incdec_r8(&mut buf.as_mut_ptr(), Reg8::A, false);
        assert_eq!(decode(&buf), "inc al");
        incdec_r8(&mut buf.as_mut_ptr(), Reg8::C, true);
        assert_eq!(decode(&buf), "dec bl");
    }

    #[test]
    fn test_mov_r8_imm() {
        let mut buf = [0; 64];
        mov_r8_imm(&mut buf.as_mut_ptr(), Reg8::A, 0x42);
        assert_eq!(decode(&buf), "mov al,42h");
    }

    #[test]
    fn test_rot() {
        let mut buf = [0; 64];
        rot(&mut buf.as_mut_ptr(), Reg8::A, Rot::Rcl);
        assert_eq!(decode(&buf), "rcl al,1");
        rot(&mut buf.as_mut_ptr(), Reg8::A, Rot::Rcr);
        assert_eq!(decode(&buf), "rcr al,1");
        rot(&mut buf.as_mut_ptr(), Reg8::A, Rot::Rol);
        assert_eq!(decode(&buf), "rol al,1");
        rot(&mut buf.as_mut_ptr(), Reg8::A, Rot::Ror);
        assert_eq!(decode(&buf), "ror al,1");

        // h
        rot(&mut buf.as_mut_ptr(), Reg8::B, Rot::Rcl);
        assert_eq!(decode(&buf), "rcl bh,1");
        rot(&mut buf.as_mut_ptr(), Reg8::B, Rot::Rcr);
        assert_eq!(decode(&buf), "rcr bh,1");
        rot(&mut buf.as_mut_ptr(), Reg8::B, Rot::Rol);
        assert_eq!(decode(&buf), "rol bh,1");
        rot(&mut buf.as_mut_ptr(), Reg8::B, Rot::Ror);
        assert_eq!(decode(&buf), "ror bh,1");

        // shifts
        rot(&mut buf.as_mut_ptr(), Reg8::A, Rot::Shr);
        assert_eq!(decode(&buf), "shr al,1");
        rot(&mut buf.as_mut_ptr(), Reg8::A, Rot::Sar);
        assert_eq!(decode(&buf), "sar al,1");
        rot(&mut buf.as_mut_ptr(), Reg8::A, Rot::Shl);
        assert_eq!(decode(&buf), "shl al,1");

        // swap
        rot4(&mut buf.as_mut_ptr(), Reg8::A);
        assert_eq!(decode(&buf), "rol al,4");
    }

    #[test]
    fn test_not_a() {
        let mut buf = [0; 64];
        not_a(&mut buf.as_mut_ptr());
        assert_eq!(decode(&buf), "not al");
    }

    #[test]
    fn test_mov_r8_r8() {
        let mut buf = [0; 64];
        mov_r8_r8(&mut buf.as_mut_ptr(), Reg8::A, Reg8::B);
        assert_eq!(decode(&buf), "mov al,bh");
        mov_r8_r8(&mut buf.as_mut_ptr(), Reg8::C, Reg8::L);
        assert_eq!(decode(&buf), "mov bl,dl");
    }

    #[test]
    fn test_alu_a_r8() {
        let mut buf = [0; 64];
        alu_a_r8(&mut buf.as_mut_ptr(), Reg8::B, AluOp::Add);
        assert_eq!(decode(&buf), "add al,bh");
        alu_a_r8(&mut buf.as_mut_ptr(), Reg8::B, AluOp::Adc);
        assert_eq!(decode(&buf), "adc al,bh");
        alu_a_r8(&mut buf.as_mut_ptr(), Reg8::B, AluOp::Sub);
        assert_eq!(decode(&buf), "sub al,bh");
        alu_a_r8(&mut buf.as_mut_ptr(), Reg8::B, AluOp::Sbb);
        assert_eq!(decode(&buf), "sbb al,bh");
        alu_a_r8(&mut buf.as_mut_ptr(), Reg8::B, AluOp::And);
        assert_eq!(decode(&buf), "and al,bh");
        alu_a_r8(&mut buf.as_mut_ptr(), Reg8::B, AluOp::Xor);
        assert_eq!(decode(&buf), "xor al,bh");
        alu_a_r8(&mut buf.as_mut_ptr(), Reg8::B, AluOp::Or);
        assert_eq!(decode(&buf), "or al,bh");
        alu_a_r8(&mut buf.as_mut_ptr(), Reg8::B, AluOp::Cmp);
        assert_eq!(decode(&buf), "cmp al,bh");
    }

    #[test]
    fn test_mov_hl_sp_imm() {
        let mut buf = [0; 64];
        mov_hl_sp_imm(&mut buf.as_mut_ptr(), 0x42);
        let mut d = Decoder::new(&buf);
        assert_eq!(d.decode(), "mov dx,si");
        assert_eq!(d.decode(), "add dx,42h");

        mov_hl_sp_imm(&mut buf.as_mut_ptr(), 0xBE);
        let mut d = Decoder::new(&buf);
        assert_eq!(d.decode(), "mov dx,si");
        assert_eq!(d.decode(), "add dx,0FFBEh");
    }

    #[test]
    fn test_alu_a_imm() {
        let mut buf = [0; 64];
        alu_a_imm(&mut buf.as_mut_ptr(), 0x42, AluOp::Add);
        assert_eq!(decode(&buf), "add al,42h");
        alu_a_imm(&mut buf.as_mut_ptr(), 0x42, AluOp::Adc);
        assert_eq!(decode(&buf), "adc al,42h");
        alu_a_imm(&mut buf.as_mut_ptr(), 0x42, AluOp::Sub);
        assert_eq!(decode(&buf), "sub al,42h");
        alu_a_imm(&mut buf.as_mut_ptr(), 0x42, AluOp::Sbb);
        assert_eq!(decode(&buf), "sbb al,42h");
        alu_a_imm(&mut buf.as_mut_ptr(), 0x42, AluOp::And);
        assert_eq!(decode(&buf), "and al,42h");
        alu_a_imm(&mut buf.as_mut_ptr(), 0x42, AluOp::Xor);
        assert_eq!(decode(&buf), "xor al,42h");
        alu_a_imm(&mut buf.as_mut_ptr(), 0x42, AluOp::Or);
        assert_eq!(decode(&buf), "or al,42h");
        alu_a_imm(&mut buf.as_mut_ptr(), 0x42, AluOp::Cmp);
        assert_eq!(decode(&buf), "cmp al,42h");
    }

    #[test]
    fn test_helpers() {
        let mut buf = [0; 64];
        movabs_r08(&mut buf.as_mut_ptr(), 0x1234_5678 as *const libc::c_void);
        assert_eq!(decode(&buf), "mov r8,12345678h");
        mov_r08addr_r16(&mut buf.as_mut_ptr(), Reg16::BC);
        assert_eq!(decode(&buf), "mov [r8],bx");
        mov_r16_r08addr(&mut buf.as_mut_ptr(), Reg16::BC);
        assert_eq!(decode(&buf), "mov bx,[r8]");
        lea_r08_add_imm8(&mut buf.as_mut_ptr(), 2);
        assert_eq!(decode(&buf), "lea r8,[r8+2]");

        mov_r08addr_r16(&mut buf.as_mut_ptr(), Reg16::HL);
        assert_eq!(decode(&buf), "mov [r8],dx");
        mov_r16_r08addr(&mut buf.as_mut_ptr(), Reg16::HL);
        assert_eq!(decode(&buf), "mov dx,[r8]");

        rot_r8_imm8(&mut buf.as_mut_ptr(), Reg8::B, 4);
        assert_eq!(decode(&buf), "shl bh,4");
        or_r8_r8(&mut buf.as_mut_ptr(), Reg8::F, Reg8::B);
        assert_eq!(decode(&buf), "or ah,bh");
    }
}
