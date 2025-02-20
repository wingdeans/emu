#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid cartridge checksum from header `0x{header:02x}` not equal to calculated `0x{calculated:02x}`")]
    InvalidCartridgeHeaderChecksum { header: u8, calculated: u8 },
    #[error("unsupported cartridge feature: {0}")]
    UnrecognizedCartridgeHeaderField(String),
    #[error("failed to load cartridge: {0}")]
    CartridgeLoadFailure(String),
}

pub type Result<T> = std::result::Result<T, Error>;
