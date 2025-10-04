use crate::protocol::packet::PacketData;
use crate::protocol::packet::client::ClientboundPacket;

#[derive(Debug)]
pub struct AddResourcePack;

impl ClientboundPacket for AddResourcePack {
    const PACKET_ID: i32 = 0x09;

    fn encode(self, _data: &mut PacketData) {
        todo!()
    }
}
