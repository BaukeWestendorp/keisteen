use std::io;

#[derive(Debug, thiserror::Error)]
pub enum CraftError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Could not decode packet")]
    InvalidPacket,
    #[error("Unexpected end of file")]
    UnexpectedEof,

    #[error("Invalid UTF-8 encoding")]
    InvalidUtf8,
    #[error("String is too long")]
    StringTooLong,
    #[error("VarInt is too big")]
    VarIntTooBig,
}
