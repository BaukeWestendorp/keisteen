use crate::error::KeisteenResult;
use crate::protocol::packet::ServerboundPacket;
use crate::server::conn::ConnectionState;
use crate::types::VarInt;

use super::RawPacket;

#[derive(Debug)]
pub enum SHandshakingPacket {
    Handshake {
        protocol_version: VarInt,
        server_address: String,
        server_port: u16,
        intent: ConnectionState,
    },
    // TODO: LegacyServerListPing
}

impl ServerboundPacket for SHandshakingPacket {
    fn decode(mut raw: RawPacket) -> KeisteenResult<Self> {
        match raw.packet_id.raw() {
            0x00 => Ok(Self::Handshake {
                protocol_version: raw.data.read()?,
                server_address: raw.data.read()?,
                server_port: raw.data.read()?,
                intent: match raw.data.read::<VarInt>()?.raw() {
                    1 => ConnectionState::Status,
                    2 => ConnectionState::Login,
                    3 => ConnectionState::Transfer,
                    _ => unreachable!("invalid intent value"),
                },
            }),
            id => Self::handle_invalid_packet_id(id),
        }
    }
}
