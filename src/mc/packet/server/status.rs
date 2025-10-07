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

        let properties = connection.server().server_folder().properties();
        let config = connection.server().server_folder().config();
        let player_list = connection.server().player_list();

        let players_sample = player_list
            .players()
            .take(12)
            .map(|p| StatusResponsePlayerSample { name: p.username(), id: p.uuid() })
            .collect();

        let response = StatusResponse {
            version: StatusResponseVersion {
                name: crate::MC_VERSION,
                protocol: crate::MC_PROTOCOL.raw(),
            },
            players: StatusResponsePlayers {
                max: properties.max_players() as i32,
                online: player_list.player_count() as i32,
                sample: players_sample,
            },
            description: TextComponent::text(properties.motd()),
            favicon: None,
            enforces_secure_chat: config.enfores_secure_chat(),
        };

        let json_response = serde_json::to_string(&response).unwrap();

        connection
            .send_packet(packet::client::status::StatusResponse { json_response: &json_response })
            .await?;

        Ok(())
    }
}

#[derive(serde::Serialize)]
pub struct StatusResponse<'a> {
    pub version: StatusResponseVersion<'a>,
    pub players: StatusResponsePlayers<'a>,
    pub description: TextComponent,
    pub favicon: Option<String>,
    pub enforces_secure_chat: bool,
}

#[derive(serde::Serialize)]
pub struct StatusResponseVersion<'a> {
    pub name: &'a str,
    pub protocol: i32,
}

#[derive(serde::Serialize)]
pub struct StatusResponsePlayers<'a> {
    pub max: i32,
    pub online: i32,
    pub sample: Vec<StatusResponsePlayerSample<'a>>,
}

#[derive(serde::Serialize)]
pub struct StatusResponsePlayerSample<'a> {
    pub name: &'a str,
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
