use bytes::Bytes;

use crate::error::KeisteenResult;
use crate::mc::packet::server::ServerboundPacket;
use crate::mc::packet::{self, ServerboundRawPacket};
use crate::mc::protocol::ProtocolRead;
use crate::server::connection::Connection;

#[derive(Debug)]
pub struct StatusRequest {}

impl ServerboundPacket for StatusRequest {
    const PACKET_ID: i32 = 0x00;

    fn decode_data(_bytes: Bytes) -> KeisteenResult<Self> {
        Ok(Self {})
    }

    async fn handle(self, connection: &mut Connection) -> KeisteenResult<()> {
        log::trace!("<<< {self:?}");

        let json_response = format!(
            r#"{{
                "version": {{
                    "name": "{version}",
                    "protocol": {protocol}
                }},
                "players": {{
                    "max": 20,
                    "online": 0,
                    "sample": []
                }},
                "description": {{
                    "text": "A Keisteen Minecraft Server"
                }},
                "enforcesSecureChat": false
            }}"#,
            version = crate::MC_VERSION,
            protocol = crate::MC_PROTOCOL
        );

        connection
            .send_packet(packet::client::status::StatusResponse { json_response: &json_response })
            .await?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct PingRequest {
    pub timestamp: i64,
}

impl ServerboundPacket for PingRequest {
    const PACKET_ID: i32 = 0x01;

    fn decode_data(mut bytes: Bytes) -> KeisteenResult<Self> {
        Ok(Self { timestamp: i64::read(&mut bytes)? })
    }

    async fn handle(self, connection: &mut Connection) -> KeisteenResult<()> {
        log::trace!("<<< {self:?}");

        connection
            .send_packet(packet::client::status::PongResponse { timestamp: self.timestamp })
            .await?;

        connection.stop().await;

        Ok(())
    }
}

pub async fn handle_raw_packet(
    packet: ServerboundRawPacket,
    connection: &mut Connection,
) -> KeisteenResult<()> {
    match packet.id.raw() {
        StatusRequest::PACKET_ID => {
            StatusRequest::decode_data(packet.data)?.handle(connection).await?;
        }
        PingRequest::PACKET_ID => {
            PingRequest::decode_data(packet.data)?.handle(connection).await?;
        }
        _ => {
            log::warn!("unknown packet id: {}", packet.id.raw());
        }
    }

    Ok(())
}
