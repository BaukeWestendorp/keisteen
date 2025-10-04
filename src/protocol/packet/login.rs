use eyre::bail;
use uuid::Uuid;

use crate::error::KeisteenError;
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
                    data.write_all(server_id);
                    data.write_all_prefixed(public_key);
                    data.write_all_prefixed(verify_token);
                    data.write_all(should_authenticate);
                    data
                },
            },
            CLoginPacket::LoginSuccess { uuid, username, .. } => RawPacket {
                packet_id: VarInt::new(0x02),
                data: {
                    let mut data = PacketData::new();
                    data.write_all(uuid);
                    data.write_all(username);
                    data.write_all_prefixed(Vec::<()>::new()); // TODO: Write properties.
                    data
                },
            },
            CLoginPacket::SetCompression { threshold } => RawPacket {
                packet_id: VarInt::new(0x03),
                data: {
                    let mut data = PacketData::new();
                    data.write_all(threshold);
                    data
                },
            },
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
    type Error = KeisteenError;

    fn try_from(mut packet: RawPacket) -> Result<Self, Self::Error> {
        match packet.packet_id.raw() {
            0x00 => Ok(SLoginPacket::LoginStart {
                name: packet.data.read()?,
                player_uuid: packet.data.read()?,
            }),
            0x01 => Ok(SLoginPacket::EncryptionResponse {
                shared_secret: packet.data.read_prefixed()?,
                verify_token: packet.data.read_prefixed()?,
            }),
            0x02 => todo!(),
            0x03 => Ok(SLoginPacket::LoginAcknowledged),
            0x04 => todo!(),
            packet_id => bail!("invalid packet id: {packet_id:#04x}"),
        }
    }
}
