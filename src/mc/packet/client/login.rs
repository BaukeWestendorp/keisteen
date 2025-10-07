use bytes::BytesMut;

use crate::mc::packet::client::ClientboundPacket;
use crate::mc::protocol::BytesMutExt;
use crate::server::game_profile::{GameProfile, GameProfileProperty};

#[derive(Debug)]
pub struct LoginSuccess {
    pub(crate) game_profile: GameProfile,
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
