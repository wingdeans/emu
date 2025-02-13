#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("bus fault accessing address `0x{addr:04x}`: {msg}")]
    BusFault { addr: u16, msg: String },
    #[error("unsupported rom size: 0x{0:02x}")]
    UnsupportedRomSize(u8),
}

pub type Result<T> = std::result::Result<T, Error>;
