use bytes::BytesMut;

use crate::mc::packet::client::ClientboundPacket;
use crate::mc::protocol::ProtocolWrite;

#[derive(Debug)]
pub struct StatusResponse<'a> {
    pub json_response: &'a str,
}

impl<'a> ClientboundPacket for StatusResponse<'a> {
    const PACKET_ID: i32 = 0x00;

    fn encode_data(self, bytes: &mut BytesMut) {
        self.json_response.write(bytes);
    }
}

#[derive(Debug)]
pub struct PongResponse {
    pub timestamp: i64,
}

impl ClientboundPacket for PongResponse {
    const PACKET_ID: i32 = 0x01;

    fn encode_data(self, bytes: &mut BytesMut) {
        self.timestamp.write(bytes);
    }
}
