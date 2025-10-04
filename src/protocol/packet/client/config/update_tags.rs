use crate::protocol::packet::PacketData;
use crate::protocol::packet::client::ClientboundPacket;

#[derive(Debug)]
pub struct UpdateTags;

impl ClientboundPacket for UpdateTags {
    const PACKET_ID: i32 = 0x0D;

    fn encode(self, _data: &mut PacketData) {
        todo!()
    }
}
