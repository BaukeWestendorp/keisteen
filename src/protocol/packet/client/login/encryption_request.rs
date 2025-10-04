use crate::protocol::packet::PacketData;
use crate::protocol::packet::client::ClientboundPacket;

#[derive(Debug)]
pub struct EncryptionRequest {
    pub server_id: String,
    pub public_key: Vec<u8>,
    pub verify_token: Vec<u8>,
    pub should_authenticate: bool,
}

impl ClientboundPacket for EncryptionRequest {
    const PACKET_ID: i32 = 0x01;

    fn encode(self, data: &mut PacketData) {
        data.write(self.server_id);
        data.write_prefixed(self.public_key);
        data.write_prefixed(self.verify_token);
        data.write(self.should_authenticate);
    }
}
