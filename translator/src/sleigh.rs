#[repr(C)]
pub struct Sleigh {
    _data: [u8; 0]
}

extern "C" {
    pub(crate) fn sleigh_new() -> *mut Sleigh;
    pub(crate) fn sleigh_free(sleigh: *mut Sleigh);
}
