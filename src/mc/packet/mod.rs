use bytes::{Bytes, BytesMut};

use crate::mc::types::VarInt;

pub mod client;
pub mod server;

#[derive(Debug)]
pub struct ClientboundRawPacket {
    pub id: VarInt,
    pub data: BytesMut,
}

impl ClientboundRawPacket {
    pub fn length(&self) -> i32 {
        self.id.len() as i32 + self.data.len() as i32
    }
}

#[derive(Debug)]
pub struct ServerboundRawPacket {
    pub id: VarInt,
    pub data: Bytes,
}
