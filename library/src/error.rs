#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("unsupported rom size: 0x{0:02x}")]
    UnsupportedRomSize(u8),
}

pub type Result<T> = std::result::Result<T, Error>;
