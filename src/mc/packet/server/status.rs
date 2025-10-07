use bytes::{Buf, Bytes};
use uuid::Uuid;

use crate::error::KeisteenResult;
use crate::mc::packet::server::ServerboundPacket;
use crate::mc::packet::{self, ServerboundRawPacket};
use crate::mc::text::TextComponent;
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

        let response = StatusResponse {
            version: StatusResponseVersion { name: crate::MC_VERSION, protocol: 763 },
            players: StatusResponsePlayers {
                max: 42,
                online: 2,
                sample: vec![
                    StatusResponsePlayerSample { name: "dinnerbone", id: Uuid::new_v4() },
                    StatusResponsePlayerSample { name: "keisteen", id: Uuid::new_v4() },
                ],
            },
            description: TextComponent::text("A Keisteen Minecraft Server"),
            favicon: None,
            enforces_secure_chat: false,
        };

        let json_response = serde_json::to_string(&response).unwrap();

        connection
            .send_packet(packet::client::status::StatusResponse { json_response: &json_response })
            .await?;

        Ok(())
    }
}

#[derive(serde::Serialize)]
pub struct StatusResponse {
    pub version: StatusResponseVersion,
    pub players: StatusResponsePlayers,
    pub description: TextComponent,
    pub favicon: Option<String>,
    pub enforces_secure_chat: bool,
}

#[derive(serde::Serialize)]
pub struct StatusResponseVersion {
    pub name: &'static str,
    pub protocol: i32,
}

#[derive(serde::Serialize)]
pub struct StatusResponsePlayers {
    pub max: i32,
    pub online: i32,
    pub sample: Vec<StatusResponsePlayerSample>,
}

#[derive(serde::Serialize)]
pub struct StatusResponsePlayerSample {
    pub name: &'static str,
    pub id: Uuid,
}

#[derive(Debug)]
pub struct PingRequest {
    pub timestamp: i64,
}

impl ServerboundPacket for PingRequest {
    const PACKET_ID: i32 = 0x01;

    fn decode_data(mut bytes: Bytes) -> KeisteenResult<Self> {
        Ok(Self { timestamp: bytes.try_get_i64()? })
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
