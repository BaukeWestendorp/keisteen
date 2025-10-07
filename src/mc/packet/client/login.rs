use bytes::BytesMut;
use uuid::Uuid;

use crate::mc::packet::client::ClientboundPacket;
use crate::mc::protocol::BytesMutExt;

#[derive(Debug)]
pub struct LoginSuccess<'a> {
    pub(crate) game_profile: GameProfile<'a>,
}

impl<'a> ClientboundPacket for LoginSuccess<'a> {
    const PACKET_ID: i32 = 0x02;

    fn encode_data(self, bytes: &mut BytesMut) {
        self.game_profile.encode_data(bytes);
    }
}

#[derive(Debug)]
pub struct GameProfile<'a> {
    pub uuid: Uuid,
    pub username: &'a str,
    pub properties: Vec<GameProfileProperty<'a>>,
}

impl<'a> GameProfile<'a> {
    fn encode_data(&self, bytes: &mut BytesMut) {
        bytes.put_uuid(&self.uuid);
        bytes.put_prefixed_string(self.username);
        bytes.put_prefixed_array(&self.properties, |prop, bytes| prop.encode_data(bytes));
    }
}

#[derive(Debug)]
pub struct GameProfileProperty<'a> {
    pub name: &'a str,
    pub value: &'a str,
    pub signature: Option<&'a str>,
}

impl<'a> GameProfileProperty<'a> {
    fn encode_data(&self, bytes: &mut BytesMut) {
        bytes.put_prefixed_string(self.name);
        bytes.put_prefixed_string(self.value);
        bytes.put_prefixed_option(&self.signature, |sig, bytes| bytes.put_prefixed_string(sig));
    }
}
