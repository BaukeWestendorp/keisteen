use crate::protocol::packet::PacketData;
use crate::protocol::packet::client::ClientboundPacket;

#[derive(Debug)]
pub struct ShowDialog;

impl ClientboundPacket for ShowDialog {
    const PACKET_ID: i32 = 0x12;

    fn encode(self, _data: &mut PacketData) {
        todo!()
    }
}
