use crate::protocol::packet::PacketData;
use crate::protocol::packet::client::ClientboundPacket;

#[derive(Debug)]
pub struct Disconnected;

impl ClientboundPacket for Disconnected {
    const PACKET_ID: i32 = 0x00;

    fn encode(self, _data: &mut PacketData) {
        todo!()
    }
}
