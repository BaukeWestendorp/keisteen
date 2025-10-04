use crate::protocol::packet::PacketData;
use crate::protocol::packet::client::ClientboundPacket;
use crate::types::{Identifier, VarInt};

#[derive(Debug)]
pub struct LoginPluginRequest {
    pub message_id: VarInt,
    pub channel: Identifier,
    pub data: Vec<u8>,
}

impl ClientboundPacket for LoginPluginRequest {
    const PACKET_ID: i32 = 0x04;

    fn encode(self, _data: &mut PacketData) {
        todo!()
    }
}
