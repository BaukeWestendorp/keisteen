use crate::protocol::packet::PacketData;
use crate::protocol::packet::client::ClientboundPacket;

#[derive(Debug)]
pub struct StatusResponse {
    pub json_response: String,
}

impl ClientboundPacket for StatusResponse {
    const PACKET_ID: i32 = 0x00;

    fn encode(self, data: &mut PacketData) {
        data.write(self.json_response);
    }
}
