use crate::mc::types::VarInt;

pub const BRAND: &str = "Keisteen";
pub const MC_VERSION: &str = "1.21.8";
pub const MC_PROTOCOL: VarInt = VarInt::new(772);

pub mod error;
pub mod mc;
pub mod server;
