#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("attempted to decode invalid operation `0b{0:08b}`")]
    InvalidOperation(u8),
    #[error("attempted to index {0}-bit field with value `{1}`")]
    InvalidIndexWidth(u8, u8),
    #[error("not implemented")]
    NotImplemented,
    #[error("bus fault accessing address `0x{0:04x}`")]
    BusFault(u16),
}

pub type Result<T> = std::result::Result<T, Error>;
