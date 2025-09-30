use crate::error::CraftError;
use crate::server::conn::ConnectionState;
use crate::types::VarInt;

use super::RawPacket;

#[derive(Debug)]
#[allow(dead_code)]
pub enum SHandshakingPacket {
    Handshake {
        protocol_version: VarInt,
        server_address: String,
        server_port: u16,
        intent: ConnectionState,
    },
    // TODO: LegacyServerListPing
}

impl TryFrom<RawPacket> for SHandshakingPacket {
    type Error = CraftError;

    fn try_from(mut packet: RawPacket) -> Result<Self, Self::Error> {
        match packet.packet_id.raw() {
            0x00 => Ok(Self::Handshake {
                protocol_version: packet.data.consume_varint()?,
                server_address: packet.data.consume_string(255)?,
                server_port: packet.data.consume_u16()?,
                intent: match packet.data.consume_varint()?.raw() {
                    1 => ConnectionState::Status,
                    2 => ConnectionState::Login,
                    3 => ConnectionState::Transfer,
                    _ => todo!(),
                },
            }),
            _ => todo!(),
        }
    }
}
