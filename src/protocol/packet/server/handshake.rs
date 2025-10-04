use crate::error::KeisteenResult;
use crate::protocol::packet::RawPacket;
use crate::protocol::packet::server::ServerboundPacket;
use crate::server::conn::{Connection, ConnectionState};
use crate::types::VarInt;

use eyre::bail;

pub fn handle_raw_packet(raw: RawPacket, conn: &mut Connection) -> KeisteenResult<()> {
    match raw.packet_id.raw() {
        Handshake::PACKET_ID => Handshake::decode(raw)?.handle(conn),
        _ => bail!("unknown handshake packet id: {}", raw.packet_id.raw()),
    }
}

#[derive(Debug)]
pub struct Handshake {
    pub protocol_version: VarInt,
    pub server_address: String,
    pub server_port: u16,
    pub intent: ConnectionState,
}

impl ServerboundPacket for Handshake {
    const PACKET_ID: i32 = 0x00;

    fn decode(mut raw: RawPacket) -> KeisteenResult<Self> {
        Ok(Self {
            protocol_version: raw.data.read()?,
            server_address: raw.data.read()?,
            server_port: raw.data.read()?,
            intent: match raw.data.read::<VarInt>()?.raw() {
                1 => ConnectionState::Status,
                2 => ConnectionState::Login,
                3 => ConnectionState::Transfer,
                _ => unreachable!("invalid intent value"),
            },
        })
    }

    fn handle(&self, conn: &mut Connection) -> KeisteenResult<()> {
        if self.protocol_version != crate::MC_PROTOCOL {
            log::warn!(
                "client has protocol version {}, but server is {}",
                self.protocol_version,
                crate::MC_PROTOCOL
            );

            // TODO: Kick player.
        }

        conn.state = self.intent;

        Ok(())
    }
}
