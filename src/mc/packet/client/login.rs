use bytes::BytesMut;
use uuid::Uuid;

use crate::mc::packet::client::ClientboundPacket;
use crate::mc::protocol::{ProtocolPrefixedWrite, ProtocolWrite};

pub struct LoginSuccess {
    pub(crate) game_profile: GameProfile,
}

impl ClientboundPacket for LoginSuccess {
    const PACKET_ID: i32 = 0x02;

    fn encode_data(self, bytes: &mut BytesMut) {
        self.game_profile.write(bytes);
    }
}

pub struct GameProfile {
    pub uuid: Uuid,
    pub username: String,
    pub properties: Vec<GameProfileProperty>,
}

impl ProtocolWrite for GameProfile {
    fn write(&self, bytes: &mut BytesMut) {
        self.uuid.write(bytes);
        self.username.write(bytes);
        self.properties.write_prefixed(bytes);
    }
}

pub struct GameProfileProperty {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}

impl ProtocolWrite for GameProfileProperty {
    fn write(&self, bytes: &mut BytesMut) {
        self.name.write(bytes);
        self.value.write(bytes);
        self.signature.write_prefixed(bytes);
    }
}
