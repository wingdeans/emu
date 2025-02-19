#[derive(Debug)]
pub struct Mapping {
    pub(crate) src: u32,
    pub(crate) srclen: u32,
    pub(crate) dst: u32,
    pub(crate) dstlen: u32,
}

pub trait Loader<E> {
    fn mappings(&self) -> Result<Vec<Mapping>, E>;
}
