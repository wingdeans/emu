use std::ptr;

extern "C" {
    fn get_addr() -> *mut libc::c_char;
}

pub fn read(addr: u16) -> u8 {
    unsafe { ptr::read(get_addr().offset(addr as isize) as *const u8) }
}

pub fn write(addr: u16, value: u8) {
    unsafe {
        ptr::write(get_addr().offset(addr as isize) as *mut u8, value);
    }
}
