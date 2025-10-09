use bytes::BytesMut;

use crate::mc::network::protocol::BytesMutExt;
use crate::mc::packet::client::ClientboundPacket;
use crate::server::game_profile::{GameProfile, GameProfileProperty};

#[derive(Debug)]
pub struct Disconnect;

impl ClientboundPacket for Disconnect {
    const PACKET_ID: i32 = 0x00;

    fn encode_data(self, _bytes: &mut BytesMut) {
        todo!()
    }
}

#[derive(Debug)]
pub struct EncryptionRequest;

impl ClientboundPacket for EncryptionRequest {
    const PACKET_ID: i32 = 0x01;

    fn encode_data(self, _bytes: &mut BytesMut) {
        todo!()
    }
}

#[derive(Debug)]
pub struct LoginSuccess {
    pub game_profile: GameProfile,
}

impl ClientboundPacket for LoginSuccess {
    const PACKET_ID: i32 = 0x02;

    fn encode_data(self, bytes: &mut BytesMut) {
        self.game_profile.encode_data(bytes);
    }
}

impl GameProfile {
    fn encode_data(&self, bytes: &mut BytesMut) {
        bytes.put_uuid(&self.uuid);
        bytes.put_prefixed_string(&self.username);
        bytes.put_prefixed_array(&self.properties, |prop, bytes| prop.encode_data(bytes));
    }
}

impl GameProfileProperty {
    fn encode_data(&self, bytes: &mut BytesMut) {
        bytes.put_prefixed_string(&self.name);
        bytes.put_prefixed_string(&self.value);
        bytes.put_prefixed_option(&self.signature, |sig, bytes| bytes.put_prefixed_string(sig));
    }
}

#[derive(Debug)]
pub struct SetCompression;

impl ClientboundPacket for SetCompression {
    const PACKET_ID: i32 = 0x03;

    fn encode_data(self, _bytes: &mut BytesMut) {
        todo!()
    }
}

#[derive(Debug)]
pub struct LoginPluginRequest;

impl ClientboundPacket for LoginPluginRequest {
    const PACKET_ID: i32 = 0x04;

    fn encode_data(self, _bytes: &mut BytesMut) {
        todo!()
    }
}

#[derive(Debug)]
pub struct CookieRequest;

impl ClientboundPacket for CookieRequest {
    const PACKET_ID: i32 = 0x05;

    fn encode_data(self, _bytes: &mut BytesMut) {
        todo!()
    }
}
