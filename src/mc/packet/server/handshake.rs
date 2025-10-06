use bytes::Bytes;

use crate::error::KeisteenResult;
use crate::mc::packet::ServerboundRawPacket;
use crate::mc::packet::server::ServerboundPacket;
use crate::mc::protocol::ProtocolRead;
use crate::mc::types::VarInt;

pub struct Handshake {
    pub protocol_version: VarInt,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: VarInt,
}

impl ServerboundPacket for Handshake {
    const PACKET_ID: i32 = 0x00;

    fn decode_data(mut bytes: Bytes) -> KeisteenResult<Self> {
        Ok(Self {
            protocol_version: VarInt::read(&mut bytes)?,
            server_address: String::read(&mut bytes)?,
            server_port: u16::read(&mut bytes)?,
            next_state: VarInt::read(&mut bytes)?,
        })
    }

    async fn handle(self) -> KeisteenResult<()> {
        log::trace!(
            "Handshake: protocol_version={}, server_address={}, server_port={}, next_state={}",
            self.protocol_version.raw(),
            self.server_address,
            self.server_port,
            self.next_state.raw()
        );

        Ok(())
    }
}

pub async fn handle_raw_packet(packet: ServerboundRawPacket) -> KeisteenResult<()> {
    match packet.id.raw() {
        Handshake::PACKET_ID => Handshake::decode_data(packet.data)?.handle().await,
        _ => {
            log::warn!("unknown packet id: {}", packet.id.raw());
            Ok(())
        }
    }
}
