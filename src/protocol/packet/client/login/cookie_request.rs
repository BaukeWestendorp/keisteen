use crate::protocol::packet::PacketData;
use crate::protocol::packet::client::ClientboundPacket;
use crate::types::Identifier;

#[derive(Debug)]
pub struct CookieRequest {
    pub key: Identifier,
}

impl ClientboundPacket for CookieRequest {
    const PACKET_ID: i32 = 0x05;

    fn encode(self, _data: &mut PacketData) {
        todo!()
    }
}
