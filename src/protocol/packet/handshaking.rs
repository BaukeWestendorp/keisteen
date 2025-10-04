use eyre::bail;

use crate::error::KeisteenError;
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

impl TryFrom<RawPacket> for SHandshakingPacket {
    type Error = KeisteenError;

    fn try_from(mut packet: RawPacket) -> Result<Self, Self::Error> {
        match packet.packet_id.raw() {
            0x00 => Ok(Self::Handshake {
                protocol_version: packet.data.read()?,
                server_address: packet.data.read()?,
                server_port: packet.data.read()?,
                intent: match packet.data.read::<VarInt>()?.raw() {
                    1 => ConnectionState::Status,
                    2 => ConnectionState::Login,
                    3 => ConnectionState::Transfer,
                    _ => unreachable!(),
                },
            }),
            packet_id => bail!("invalid packet id: {packet_id:#04x}"),
        }
    }
}
