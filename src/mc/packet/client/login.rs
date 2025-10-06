use bytes::BytesMut;
use uuid::Uuid;

use crate::mc::packet::client::ClientboundPacket;
use crate::mc::protocol::{ProtocolPrefixedWrite, ProtocolWrite};

#[derive(Debug)]
pub struct LoginSuccess<'a> {
    pub(crate) game_profile: GameProfile<'a>,
}

impl<'a> ClientboundPacket for LoginSuccess<'a> {
    const PACKET_ID: i32 = 0x02;

    fn encode_data(self, bytes: &mut BytesMut) {
        self.game_profile.write(bytes);
    }
}

#[derive(Debug)]
pub struct GameProfile<'a> {
    pub uuid: Uuid,
    pub username: &'a str,
    pub properties: Vec<GameProfileProperty<'a>>,
}

impl<'a> ProtocolWrite for GameProfile<'a> {
    fn write(&self, bytes: &mut BytesMut) {
        self.uuid.write(bytes);
        self.username.write(bytes);
        self.properties.write_prefixed(bytes);
    }
}

#[derive(Debug)]
pub struct GameProfileProperty<'a> {
    pub name: &'a str,
    pub value: &'a str,
    pub signature: Option<&'a str>,
}

impl<'a> ProtocolWrite for GameProfileProperty<'a> {
    fn write(&self, bytes: &mut BytesMut) {
        self.name.write(bytes);
        self.value.write(bytes);
        self.signature.write_prefixed(bytes);
    }
}
