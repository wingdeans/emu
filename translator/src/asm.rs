#[derive(Clone, Copy)]
pub(crate) enum Reg16 {
    AF = 0,
    BC = 3,
    DE = 1,
    HL = 2,
    SP = 6,
}

#[derive(Clone, Copy)]
pub(crate) enum Reg8 {
    A = 0,
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
    Cmp = 0x38
}

fn write(dst: &mut *mut u8, src: &[u8]) {
    unsafe {
        std::ptr::copy(src.as_ptr(), *dst, src.len());
        *dst = dst.add(src.len());
    }
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
            0xFF, // INC r/m16
            0b11_000_000 | reg as u8 | ((dec as u8) << 3),
        ],
    )
}

pub(crate) fn incdec_r8(buf: &mut *mut u8, reg: Reg8, dec: bool) {
    write(
        buf,
        &[
            0xFE, // INC r/m8
            0b11_000_000 | reg as u8 | ((dec as u8) << 3),
        ],
    )
}

pub(crate) fn mov_r8_imm(buf: &mut *mut u8, reg: Reg8, imm: u8) {
    write(
        buf,
        &[
            0xB0 + reg as u8, // MOV r8, imm8
            imm
        ],
    )
}

pub(crate) fn rot(buf: &mut *mut u8, reg: Reg8, rot: Rot) {
    write(
        buf,
        &[
            0xD0, // r/m8, 1
            0b11_000_000 | ((rot as u8) << 3) | reg as u8
        ]
    )
}

pub(crate) fn not_a(buf: &mut *mut u8) {
    write(
        buf,
        &[
            0xF6, // NOT r/m8
            0b11_010_000 | Reg8::A as u8
        ]
    )
}

pub(crate) fn mov_r8_r8(buf: &mut *mut u8, dst: Reg8, src: Reg8) {
    write(
        buf,
        &[
            0x88, // MOV r/m8, r8
            0b11_000_000 | ((src as u8) << 3) | dst as u8
        ],
    )
}

pub(crate) fn alu_a_r8(buf: &mut *mut u8, src: Reg8, op: AluOp) {
    write(
        buf,
        &[
            op as u8, // r/m8, r8
            0b11_000_000 | ((src as u8) << 3) | Reg8::A as u8
        ],
    )
}

pub(crate) fn mov_r16_r16(buf: &mut *mut u8, dst: Reg16, src: Reg16) {
    write(
        buf,
        &[
            0x66, // prefix
            0x89, // MOV r/m16, r16
            0b11_000_000 | ((src as u8) << 3) | dst as u8,
        ]
    )
}

pub(crate) fn mov_hl_sp_imm(buf: &mut *mut u8, imm: u8) {
    mov_r16_r16(buf, Reg16::HL, Reg16::SP);
    write(
        buf,
        &[
            0x66, // prefix
            0x81, // ADD r/m16, imm8
            0b11_000_000 | Reg16::HL as u8,
            imm
        ]
    );
}

pub(crate) fn rot4(buf: &mut *mut u8, reg: Reg8) {
    write(
        buf,
        &[
            0xC0, // ROL r/m8, imm8
            0b11_000_000 | reg as u8,
            4
        ]
    )
}

pub(crate) fn alu_a_imm(buf: &mut *mut u8, imm: u8, op: AluOp) {
    write(
        buf,
        &[
            4 + op as u8, // r/m8, r8
            imm
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Decoder<'a>(iced_x86::Decoder<'a>, iced_x86::IntelFormatter);

    impl<'a> Decoder<'a> {
        fn new(buf: &'a [u8]) -> Self {
            Decoder(
                iced_x86::Decoder::new(64, buf, iced_x86::DecoderOptions::NONE),
                iced_x86::IntelFormatter::new()
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
        assert_eq!(decode(&buf), "inc bx");
        incdec_r16(&mut buf.as_mut_ptr(), Reg16::BC, true);
        assert_eq!(decode(&buf), "dec bx");
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
        assert_eq!(d.decode(), "add dx,0BEh");
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
}
