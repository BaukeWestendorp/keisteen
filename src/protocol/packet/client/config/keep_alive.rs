use crate::protocol::packet::PacketData;
use crate::protocol::packet::client::ClientboundPacket;

#[derive(Debug)]
pub struct KeepAlive;

impl ClientboundPacket for KeepAlive {
    const PACKET_ID: i32 = 0x04;

    fn encode(self, _data: &mut PacketData) {
        todo!()
    }
}
