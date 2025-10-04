use crate::protocol::packet::PacketData;
use crate::protocol::packet::client::ClientboundPacket;

#[derive(Debug)]
pub struct ServerLinks;

impl ClientboundPacket for ServerLinks {
    const PACKET_ID: i32 = 0x10;

    fn encode(self, _data: &mut PacketData) {
        todo!()
    }
}
