use uuid::Uuid;

use crate::mc::protocol::packet::PacketData;
use crate::mc::protocol::packet::client::ClientboundPacket;
use crate::mc::types::{Identifier, VarInt};

#[derive(Debug)]
pub struct Disconnected;

impl ClientboundPacket for Disconnected {
    const PACKET_ID: i32 = 0x00;

    fn encode(self, _data: &mut PacketData) {
        todo!()
    }
}

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

#[derive(Debug)]
pub struct LoginSuccess {
    pub uuid: Uuid,
    pub username: String,
    pub properties: (),
}

impl ClientboundPacket for LoginSuccess {
    const PACKET_ID: i32 = 0x02;

    fn encode(self, data: &mut PacketData) {
        data.write(self.uuid);
        data.write(self.username);
        data.write_prefixed(Vec::<()>::new()); // TODO: Write properties.
    }
}

#[derive(Debug)]
pub struct SetCompression {
    pub threshold: VarInt,
}

impl ClientboundPacket for SetCompression {
    const PACKET_ID: i32 = 0x03;

    fn encode(self, data: &mut PacketData) {
        data.write(self.threshold);
    }
}

#[derive(Debug)]
pub struct LoginPluginRequest {
    pub message_id: VarInt,
    pub channel: Identifier,
    pub data: Vec<u8>,
}

impl ClientboundPacket for LoginPluginRequest {
    const PACKET_ID: i32 = 0x04;

    fn encode(self, _data: &mut PacketData) {
        todo!()
    }
}

#[derive(Debug)]
pub struct CookieRequest {
    pub key: Identifier,
}

impl ClientboundPacket for CookieRequest {
    const PACKET_ID: i32 = 0x05;

    fn encode(self, _data: &mut PacketData) {
        todo!()
    }
}
