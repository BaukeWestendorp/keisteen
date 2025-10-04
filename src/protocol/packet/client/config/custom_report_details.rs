use crate::protocol::packet::PacketData;
use crate::protocol::packet::client::ClientboundPacket;

#[derive(Debug)]
pub struct CustomReportDetails;

impl ClientboundPacket for CustomReportDetails {
    const PACKET_ID: i32 = 0x0F;

    fn encode(self, _data: &mut PacketData) {
        todo!()
    }
}
