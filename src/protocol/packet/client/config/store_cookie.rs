use crate::protocol::packet::PacketData;
use crate::protocol::packet::client::ClientboundPacket;

#[derive(Debug)]
pub struct StoreCookie;

impl ClientboundPacket for StoreCookie {
    const PACKET_ID: i32 = 0x0A;

    fn encode(self, _data: &mut PacketData) {
        todo!()
    }
}
