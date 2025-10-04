use crate::protocol::packet::PacketData;
use crate::protocol::packet::client::ClientboundPacket;

#[derive(Debug)]
pub struct Transfer;

impl ClientboundPacket for Transfer {
    const PACKET_ID: i32 = 0x0B;

    fn encode(self, _data: &mut PacketData) {
        todo!()
    }
}
