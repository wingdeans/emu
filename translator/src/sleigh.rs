use core::ffi::c_char;

mod ffi {
    use core::ffi::c_char;

    #[repr(C)]
    pub(super) struct Sleigh {
        _data: [u8; 0],
    }

    extern "C" {
        pub(super) fn sleigh_new(str: *const c_char, len: usize) -> *mut Sleigh;
        pub(super) fn sleigh_free(sleigh: *mut Sleigh);
        pub(super) fn sleigh_disassemble(
            sleigh: *mut Sleigh,
            buf: *const u8,
            len: usize,
            addr: usize,
        );
        pub(super) fn sleigh_pcode(sleigh: *mut Sleigh, buf: *const u8, len: usize, addr: usize);
    }
}

// SLEIGH

pub struct Sleigh(*mut ffi::Sleigh);

impl Sleigh {
    pub fn new(path: &str) -> Self {
        let path_ptr = path.as_ptr() as *const c_char;
        Self(unsafe { ffi::sleigh_new(path_ptr, path.len()) })
    }

    pub fn disassemble(&self, buf: &[u8]) {
        assert_ne!(0, buf.len());
        let len = std::cmp::min(16, buf.len());
        unsafe { ffi::sleigh_disassemble(self.0, buf.as_ptr(), len, 0) }
    }

    pub fn pcode(&self, buf: &[u8]) {
        assert_ne!(0, buf.len());
        let len = std::cmp::min(16, buf.len());
        unsafe { ffi::sleigh_pcode(self.0, buf.as_ptr(), len, 0) }
    }
}

impl Drop for Sleigh {
    fn drop(&mut self) {
        unsafe { ffi::sleigh_free(self.0) }
    }
}
