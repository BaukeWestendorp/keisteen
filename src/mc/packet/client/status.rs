use bytes::{BufMut, BytesMut};

use crate::mc::network::protocol::BytesMutExt;
use crate::mc::packet::client::ClientboundPacket;

#[derive(Debug)]
pub struct StatusResponse<'a> {
    pub json_response: &'a str,
}

impl<'a> ClientboundPacket for StatusResponse<'a> {
    const PACKET_ID: i32 = 0x00;

    fn encode_data(self, bytes: &mut BytesMut) {
        bytes.put_prefixed_string(self.json_response);
    }
}

#[derive(Debug)]
pub struct PongResponse {
    pub timestamp: i64,
}

impl ClientboundPacket for PongResponse {
    const PACKET_ID: i32 = 0x01;

    fn encode_data(self, bytes: &mut BytesMut) {
        bytes.put_i64(self.timestamp);
    }
}
