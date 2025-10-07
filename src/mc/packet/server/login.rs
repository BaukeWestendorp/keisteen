use bytes::Bytes;
use uuid::Uuid;

use crate::error::KeisteenResult;
use crate::mc::packet::ServerboundRawPacket;
use crate::mc::packet::server::ServerboundPacket;
use crate::mc::protocol::ProtocolRead;
use crate::server::connection::Connection;

#[derive(Debug)]
pub struct LoginStart {
    pub _name: String,
    pub _player_uuid: Uuid,
}

impl ServerboundPacket for LoginStart {
    const PACKET_ID: i32 = 0x00;

    fn decode_data(mut bytes: Bytes) -> KeisteenResult<Self> {
        Ok(Self { _name: String::read(&mut bytes)?, _player_uuid: Uuid::read(&mut bytes)? })
    }

    async fn handle(self, _connection: &mut Connection) -> KeisteenResult<()> {
        log::trace!("<<< {self:?}");
        Ok(())
    }
}

#[derive(Debug)]
pub struct EncryptionResponse {
    pub _shared_secret: [u8; 16],
    pub _verify_token: [u8; 4],
}

impl ServerboundPacket for EncryptionResponse {
    const PACKET_ID: i32 = 0x01;

    fn decode_data(mut bytes: Bytes) -> KeisteenResult<Self> {
        Ok(Self {
            _shared_secret: <[u8; 16]>::read(&mut bytes)?,
            _verify_token: <[u8; 4]>::read(&mut bytes)?,
        })
    }

    async fn handle(self, _connection: &mut Connection) -> KeisteenResult<()> {
        log::trace!("<<< {self:?}");
        Ok(())
    }
}

#[derive(Debug)]
pub struct LoginPluginResponse;

impl ServerboundPacket for LoginPluginResponse {
    const PACKET_ID: i32 = 0x02;

    fn decode_data(_bytes: Bytes) -> KeisteenResult<Self> {
        todo!()
    }

    async fn handle(self, _connection: &mut Connection) -> KeisteenResult<()> {
        todo!()
    }
}

#[derive(Debug)]
pub struct LoginAcknowledged {}

impl ServerboundPacket for LoginAcknowledged {
    const PACKET_ID: i32 = 0x03;

    fn decode_data(_bytes: Bytes) -> KeisteenResult<Self> {
        Ok(Self {})
    }

    async fn handle(self, _connection: &mut Connection) -> KeisteenResult<()> {
        log::trace!("<<< {self:?}");
        Ok(())
    }
}

#[derive(Debug)]
pub struct CookieResponse;

impl ServerboundPacket for CookieResponse {
    const PACKET_ID: i32 = 0x04;

    fn decode_data(_bytes: Bytes) -> KeisteenResult<Self> {
        todo!()
    }

    async fn handle(self, _connection: &mut Connection) -> KeisteenResult<()> {
        todo!()
    }
}

pub async fn handle_raw_packet(
    packet: ServerboundRawPacket,
    connection: &mut Connection,
) -> KeisteenResult<()> {
    match packet.id.raw() {
        LoginStart::PACKET_ID => {
            LoginStart::decode_data(packet.data)?.handle(connection).await?;
        }
        EncryptionResponse::PACKET_ID => {
            EncryptionResponse::decode_data(packet.data)?.handle(connection).await?;
        }
        LoginPluginResponse::PACKET_ID => {
            LoginPluginResponse::decode_data(packet.data)?.handle(connection).await?;
        }
        LoginAcknowledged::PACKET_ID => {
            LoginAcknowledged::decode_data(packet.data)?.handle(connection).await?;
        }
        CookieResponse::PACKET_ID => {
            CookieResponse::decode_data(packet.data)?.handle(connection).await?;
        }
        _ => {
            log::warn!("unknown packet id: {}", packet.id.raw());
        }
    }

    Ok(())
}
