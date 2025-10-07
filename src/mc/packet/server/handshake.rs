use bytes::Bytes;

use crate::error::KeisteenResult;
use crate::mc::packet::ServerboundRawPacket;
use crate::mc::packet::server::ServerboundPacket;
use crate::mc::protocol::ProtocolRead;
use crate::mc::types::VarInt;
use crate::server::connection::{Connection, ConnectionState};

#[derive(Debug)]
pub struct Handshake {
    pub protocol_version: VarInt,
    pub server_address: String,
    pub server_port: u16,
    pub intent: VarInt,
}

impl ServerboundPacket for Handshake {
    const PACKET_ID: i32 = 0x00;

    fn decode_data(mut bytes: Bytes) -> KeisteenResult<Self> {
        Ok(Self {
            protocol_version: VarInt::read(&mut bytes)?,
            server_address: String::read(&mut bytes)?,
            server_port: u16::read(&mut bytes)?,
            intent: VarInt::read(&mut bytes)?,
        })
    }

    async fn handle(self, connection: &mut Connection) -> KeisteenResult<()> {
        log::trace!("<<< {self:?}");

        connection.set_state(match self.intent.raw() {
            1 => ConnectionState::Status,
            2 => ConnectionState::Login,
            3 => ConnectionState::Transfer,
            _ => {
                log::warn!("unknown next state: {}", self.intent.raw());
                connection.stop().await;
                return Ok(());
            }
        });

        Ok(())
    }
}

pub async fn handle_raw_packet(
    packet: ServerboundRawPacket,
    connection: &mut Connection,
) -> KeisteenResult<()> {
    match packet.id.raw() {
        Handshake::PACKET_ID => {
            Handshake::decode_data(packet.data)?.handle(connection).await?;
        }
        _ => {
            log::warn!("unknown packet id: {}", packet.id.raw());
        }
    }

    Ok(())
}
