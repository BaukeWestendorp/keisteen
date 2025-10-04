use crate::protocol::packet::PacketData;
use crate::protocol::packet::client::ClientboundPacket;

#[derive(Debug)]
pub struct RemoveResourcePack;

impl ClientboundPacket for RemoveResourcePack {
    const PACKET_ID: i32 = 0x08;

    fn encode(self, _data: &mut PacketData) {
        todo!()
    }
}
