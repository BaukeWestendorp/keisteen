use crate::protocol::packet::PacketData;
use crate::protocol::packet::client::ClientboundPacket;

#[derive(Debug)]
pub struct ClearDialog;

impl ClientboundPacket for ClearDialog {
    const PACKET_ID: i32 = 0x11;

    fn encode(self, _data: &mut PacketData) {
        todo!()
    }
}
