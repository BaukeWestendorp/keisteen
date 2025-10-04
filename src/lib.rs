use crate::types::VarInt;

pub const BRAND: &str = "Keisteen";
pub const MC_VERSION: &str = "1.18.2";
pub const MC_PROTOCOL: VarInt = VarInt::new(772);

pub mod error;
pub mod nbt;
pub mod protocol;
pub mod server;
pub mod text;
pub mod types;
pub mod worldgen;
