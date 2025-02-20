#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("bus fault accessing address `0x{addr:04x}`: {msg}")]
    BusFault { addr: u16, msg: String },
    #[error("invalid cartridge checksum from header `0x{header:02x}` not equal to calculated `0x{calculated:02x}`")]
    InvalidCartridgeHeaderChecksum { header: u8, calculated: u8 },
    #[error("unsupported cartridge feature: {0}")]
    UnrecognizedCartridgeHeaderField(String),
    #[error("failed to load cartridge: {0}")]
    CartridgeLoadFailure(String),
    #[error("out of bound bank access: {0}")]
    OutOfBoundsBank(u32),
}

pub type Result<T> = std::result::Result<T, Error>;
