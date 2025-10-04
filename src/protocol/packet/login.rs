use uuid::Uuid;

use crate::error::KeisteenResult;
use crate::protocol::packet::{ClientboundPacket, ServerboundPacket};
use crate::types::{Identifier, VarInt};

use super::{PacketData, RawPacket};

#[derive(Debug)]
pub enum CLoginPacket {
    Disconnected, // TODO,
    EncryptionRequest {
        server_id: String,
        public_key: Vec<u8>,
        verify_token: Vec<u8>,
        should_authenticate: bool,
    },
    LoginSuccess {
        uuid: Uuid,
        username: String,
        // TODO
        properties: (),
    },
    SetCompression {
        threshold: VarInt,
    },
    LoginPluginRequest {
        message_id: VarInt,
        channel: Identifier,
        data: Vec<u8>,
    },
    CookieRequest {
        key: Identifier,
    },
}

impl ClientboundPacket for CLoginPacket {
    fn encode(self, data: &mut PacketData) {
        match self {
            CLoginPacket::Disconnected { .. } => todo!(),
            CLoginPacket::EncryptionRequest {
                server_id,
                public_key,
                verify_token,
                should_authenticate,
            } => {
                data.write(server_id);
                data.write_prefixed(public_key);
                data.write_prefixed(verify_token);
                data.write(should_authenticate);
            }
            CLoginPacket::LoginSuccess { uuid, username, .. } => {
                data.write(uuid);
                data.write(username);
                data.write_prefixed(Vec::<()>::new()); // TODO: Write properties.
            }
            CLoginPacket::SetCompression { threshold } => {
                data.write(threshold);
            }
            CLoginPacket::LoginPluginRequest { .. } => todo!(),
            CLoginPacket::CookieRequest { .. } => todo!(),
        }
    }

    fn packet_id(&self) -> i32 {
        match self {
            CLoginPacket::Disconnected { .. } => 0x00,
            CLoginPacket::EncryptionRequest { .. } => 0x01,
            CLoginPacket::LoginSuccess { .. } => 0x02,
            CLoginPacket::SetCompression { .. } => 0x03,
            CLoginPacket::LoginPluginRequest { .. } => 0x04,
            CLoginPacket::CookieRequest { .. } => 0x05,
        }
    }
}

#[derive(Debug)]
pub enum SLoginPacket {
    LoginStart { name: String, player_uuid: Uuid },
    EncryptionResponse { shared_secret: Vec<u8>, verify_token: Vec<u8> },
    LoginPluginResponse { message_id: VarInt, data: Vec<u8> },
    LoginAcknowledged,
    CookieResponse { key: Identifier, payload: Vec<u8> },
}

impl ServerboundPacket for SLoginPacket {
    fn decode(mut raw: RawPacket) -> KeisteenResult<Self> {
        match raw.packet_id.raw() {
            0x00 => Ok(SLoginPacket::LoginStart {
                name: raw.data.read()?,
                player_uuid: raw.data.read()?,
            }),
            0x01 => Ok(SLoginPacket::EncryptionResponse {
                shared_secret: raw.data.read_prefixed()?,
                verify_token: raw.data.read_prefixed()?,
            }),
            0x02 => todo!(),
            0x03 => Ok(SLoginPacket::LoginAcknowledged),
            0x04 => todo!(),
            id => Self::handle_invalid_packet_id(id),
        }
    }
}
