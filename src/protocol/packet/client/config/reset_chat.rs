use crate::protocol::packet::PacketData;
use crate::protocol::packet::client::ClientboundPacket;

#[derive(Debug)]
pub struct ResetChat;

impl ClientboundPacket for ResetChat {
    const PACKET_ID: i32 = 0x06;

    fn encode(self, _data: &mut PacketData) {
        todo!()
    }
}
