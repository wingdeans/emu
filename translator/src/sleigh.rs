use crate::pcode::Pcode;
use core::ffi::{c_char, c_void};

pub(crate) mod ffi {
    use crate::pcode::{Pcode, PcodeOp, Var};
    use core::ffi::{c_char, c_void};

    #[repr(C)]
    pub(super) struct Sleigh {
        _data: [u8; 0],
    }

    #[repr(C)]
    #[derive(Debug)]
    struct CVarnode {
        pub(crate) addr: u32,
        pub(crate) size: u32,
        pub(crate) offset: u64,
    }

    extern "C" {
        pub(super) fn sleigh_new(str: *const c_char, len: usize) -> *mut Sleigh;
        pub(super) fn sleigh_free(sleigh: *mut Sleigh);
        pub(super) fn sleigh_disassemble(
            sleigh: *mut Sleigh,
            buf: *const u8,
            len: usize,
            addr: u64,
            // out
            out_mnem: *mut *mut c_void,
            out_body: *mut *mut c_void,
        );
        pub(super) fn sleigh_pcode(
            sleigh: *mut Sleigh,
            buf: *const u8,
            len: usize,
            addr: u64,
            vec: *mut c_void,
        ) -> u32;
    }

    #[no_mangle]
    extern "C" fn rust_str(buf: *const c_char, len: usize) -> *mut String {
        unsafe {
            let slice = std::slice::from_raw_parts(buf, len);
            let bytes = slice.iter().map(|c| *c as u8).collect();
            let s = String::from_utf8(bytes).unwrap();
            let bs = Box::new(s);
            Box::leak(bs)
        }
    }

    #[no_mangle]
    extern "C" fn rust_push_pcode(vec: *mut Vec<Pcode>, opc: u8, vars: *mut CVarnode, size: u32) {
        let cvars = unsafe { std::slice::from_raw_parts(vars, size as usize) };
        let vars: Box<[_]> = cvars
            .into_iter()
            .map(|cv| {
                if cv.addr == std::u32::MAX {
                    None
                } else {
                    Some(Var {
                        addr: cv.addr.try_into().unwrap(),
                        size: cv.size,
                        offset: cv.offset,
                    })
                }
            })
            .collect();
        let vec = unsafe { vec.as_mut().unwrap() };

        macro_rules! let_match {
            [$($pat:pat_param),*] => {
                let [$($pat),*] = vars[..] else { unreachable!("{:?}", vars) };
            }
        }

        macro_rules! unary {
            ($constructor:path) => {{
                let_match![Some(dst), Some(src)];
                vec.push($constructor(dst, src));
            }};
        }

        macro_rules! binary {
            ($constructor:path) => {{
                let_match![Some(dst), Some(a), Some(b)];
                vec.push($constructor(dst, a, b));
            }};
        }

        match PcodeOp::from(opc) {
            PcodeOp::COPY => unary!(Pcode::Copy),
            PcodeOp::LOAD => binary!(Pcode::Load),
            PcodeOp::STORE => {
                let_match![None, Some(a), Some(b), Some(c)];
                vec.push(Pcode::Store(a, b, c))
            }
            PcodeOp::BRANCH => {
                let_match![None, Some(target)];
                vec.push(Pcode::Branch(target))
            }
            PcodeOp::CBRANCH => {
                let_match![None, Some(target), Some(cond)];
                vec.push(Pcode::CBranch(target, cond))
            }
            PcodeOp::CALL => {
                let_match![None, Some(target)];
                vec.push(Pcode::Call(target))
            }
            PcodeOp::RETURN => {
                let_match![None, Some(target)];
                vec.push(Pcode::Return)
            }
            PcodeOp::CALLOTHER => vec.push(Pcode::CallOther),
            // int
            PcodeOp::INT_EQUAL => binary!(Pcode::IntEQ),
            PcodeOp::INT_ZEXT => unary!(Pcode::IntZext),
            PcodeOp::INT_ADD => binary!(Pcode::IntAdd),
            PcodeOp::INT_SUB => binary!(Pcode::IntSub),
            PcodeOp::INT_XOR => binary!(Pcode::IntXor),
            PcodeOp::INT_AND => binary!(Pcode::IntAnd),
            PcodeOp::INT_OR => binary!(Pcode::IntOr),
            PcodeOp::INT_LEFT => binary!(Pcode::IntLeft),
            PcodeOp::INT_RIGHT => binary!(Pcode::IntRight),
            // binary
            PcodeOp::BOOL_NEGATE => unary!(Pcode::BoolNegate),
            op => todo!("Unhandled Pcode opcode: {:?}", op),
        }
    }
}

// SLEIGH

pub struct Sleigh(*mut ffi::Sleigh);

impl Sleigh {
    pub fn new(path: &str) -> Self {
        let path_ptr = path.as_ptr() as *const c_char;
        Self(unsafe { ffi::sleigh_new(path_ptr, path.len()) })
    }

    pub fn disassemble(&self, buf: &[u8], addr: u64) -> (String, String) {
        assert_ne!(0, buf.len());
        let len = std::cmp::min(16, buf.len());
        let mut mnem: *mut String = std::ptr::null_mut();
        let mut body: *mut String = std::ptr::null_mut();
        unsafe {
            ffi::sleigh_disassemble(
                self.0,
                buf.as_ptr(),
                len,
                addr,
                (&mut mnem as *mut *mut String) as *mut *mut c_void,
                (&mut body as *mut *mut String) as *mut *mut c_void,
            );
            (*Box::from_raw(mnem), *Box::from_raw(body))
        }
    }

    pub fn pcode(&self, buf: &[u8], addr: u64, vec: &mut Vec<Pcode>) -> u32 {
        assert_ne!(0, buf.len());
        let len = std::cmp::min(16, buf.len());
        unsafe {
            ffi::sleigh_pcode(
                self.0,
                buf.as_ptr(),
                len,
                addr,
                (vec as *mut Vec<Pcode>) as *mut c_void,
            )
        }
    }
}

impl Drop for Sleigh {
    fn drop(&mut self) {
        unsafe { ffi::sleigh_free(self.0) }
    }
}
