use uuid::Uuid;

use crate::error::CraftError;
use crate::types::{Identifier, JsonTextComponent, VarInt};

use super::{PacketData, RawPacket};

#[derive(Debug)]
pub enum CLoginPacket {
    Disconnected {
        reason: JsonTextComponent,
    },
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

impl From<CLoginPacket> for RawPacket {
    fn from(packet: CLoginPacket) -> Self {
        match packet {
            CLoginPacket::Disconnected { .. } => todo!(),
            CLoginPacket::EncryptionRequest {
                server_id,
                public_key,
                verify_token,
                should_authenticate,
            } => RawPacket {
                packet_id: VarInt::new(0x01),
                data: {
                    let mut data = PacketData::new();
                    data.write_string(server_id, 20);
                    data.write_prefixed_byte_array(public_key);
                    data.write_prefixed_byte_array(verify_token);
                    data.write_bool(should_authenticate);
                    data
                },
            },
            CLoginPacket::LoginSuccess { uuid, username, .. } => RawPacket {
                packet_id: VarInt::new(0x02),
                data: {
                    let mut data = PacketData::new();
                    data.write_uuid(uuid);
                    data.write_string(username, 16);
                    data.write_prefixed_byte_array(Vec::new()); // TODO: Write properties.
                    data
                },
            },
            CLoginPacket::SetCompression { .. } => todo!(),
            CLoginPacket::LoginPluginRequest { .. } => todo!(),
            CLoginPacket::CookieRequest { .. } => todo!(),
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

impl TryFrom<RawPacket> for SLoginPacket {
    type Error = CraftError;

    fn try_from(mut packet: RawPacket) -> Result<Self, Self::Error> {
        match packet.packet_id.raw() {
            0x00 => Ok(SLoginPacket::LoginStart {
                name: packet.data.consume_string(16)?,
                player_uuid: packet.data.consume_uuid()?,
            }),
            0x01 => Ok(SLoginPacket::EncryptionResponse {
                shared_secret: packet.data.consume_prefixed_byte_array()?,
                verify_token: packet.data.consume_prefixed_byte_array()?,
            }),
            0x02 => todo!("LoginPluginResponse"),
            0x03 => Ok(SLoginPacket::LoginAcknowledged),
            0x04 => todo!("CookieResponse"),
            _ => return Err(CraftError::InvalidPacket),
        }
    }
}
