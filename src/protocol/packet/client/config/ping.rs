use crate::protocol::packet::PacketData;
use crate::protocol::packet::client::ClientboundPacket;

#[derive(Debug)]
pub struct Ping;

impl ClientboundPacket for Ping {
    const PACKET_ID: i32 = 0x05;

    fn encode(self, _data: &mut PacketData) {
        todo!()
    }
}
