use crate::protocol::packet::PacketData;
use crate::protocol::packet::client::ClientboundPacket;

#[derive(Debug)]
pub struct PongResponse {
    pub timestamp: i64,
}

impl ClientboundPacket for PongResponse {
    const PACKET_ID: i32 = 0x01;

    fn encode(self, data: &mut PacketData) {
        data.write(self.timestamp);
    }
}
