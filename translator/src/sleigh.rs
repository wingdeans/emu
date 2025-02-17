use core::ffi::c_char;

mod ffi {
    use core::ffi::c_char;

    #[repr(C)]
    pub(super) struct Sla {
        _data: [u8; 0],
    }

    #[repr(C)]
    pub(super) struct Sleigh {
        _data: [u8; 0],
    }

    extern "C" {
        pub(super) fn sla_new(str: *const c_char, len: usize) -> *mut Sla;
        pub(super) fn sla_free(sla: *mut Sla);

        pub(super) fn sleigh_new(str: *const c_char, len: usize) -> *mut Sleigh;
        pub(super) fn sleigh_free(sleigh: *mut Sleigh);
    }
}

// SLA

pub struct Sla(*mut ffi::Sla);

impl Sla {
    pub(crate) fn new(path: &str) -> Self {
        let path_ptr = path.as_ptr() as *const c_char;
        Self(unsafe { ffi::sla_new(path_ptr, path.len()) })
    }
}

impl Drop for Sla {
    fn drop(&mut self) {
        unsafe { ffi::sla_free(self.0) }
    }
}

// SLEIGH

pub struct Sleigh(*mut ffi::Sleigh);

impl Sleigh {
    pub(crate) fn new(path: &str) -> Self {
        let path_ptr = path.as_ptr() as *const c_char;
        Self(unsafe { ffi::sleigh_new(path_ptr, path.len()) })
    }
}

impl Drop for Sleigh {
    fn drop(&mut self) {
        unsafe { ffi::sleigh_free(self.0) }
    }
}
