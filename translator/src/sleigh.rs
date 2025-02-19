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
            // out
            out_mnem: *mut *mut String,
            out_body: *mut *mut String,
        );
        pub(super) fn sleigh_pcode(sleigh: *mut Sleigh, buf: *const u8, len: usize, addr: usize);
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
}

// SLEIGH

pub struct Sleigh(*mut ffi::Sleigh);

impl Sleigh {
    pub fn new(path: &str) -> Self {
        let path_ptr = path.as_ptr() as *const c_char;
        Self(unsafe { ffi::sleigh_new(path_ptr, path.len()) })
    }

    pub fn disassemble(&self, buf: &[u8], addr: usize) -> (String, String) {
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
                &mut mnem as *mut *mut String,
                &mut body as *mut *mut String,
            );
            (*Box::from_raw(mnem), *Box::from_raw(body))
        }
    }

    pub fn pcode(&self, buf: &[u8], addr: usize) {
        assert_ne!(0, buf.len());
        let len = std::cmp::min(16, buf.len());
        unsafe { ffi::sleigh_pcode(self.0, buf.as_ptr(), len, addr) }
    }
}

impl Drop for Sleigh {
    fn drop(&mut self) {
        unsafe { ffi::sleigh_free(self.0) }
    }
}
