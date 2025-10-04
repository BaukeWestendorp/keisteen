use crate::error::KeisteenResult;
use crate::mc::protocol::packet::server::ServerboundPacket;
use crate::mc::protocol::packet::{RawPacket, client};
use crate::mc::text::text_component::TextComponent;
use crate::server::conn::Connection;

use eyre::bail;
use uuid::Uuid;

pub fn handle_raw_packet(raw: RawPacket, conn: &mut Connection) -> KeisteenResult<()> {
    match raw.packet_id.raw() {
        StatusRequest::PACKET_ID => StatusRequest::decode(raw)?.handle(conn),
        PingRequest::PACKET_ID => PingRequest::decode(raw)?.handle(conn),
        _ => bail!("unknown status packet id: {}", raw.packet_id.raw()),
    }
}

#[derive(Debug)]
pub struct StatusRequest;

impl ServerboundPacket for StatusRequest {
    const PACKET_ID: i32 = 0x00;

    fn decode(_raw: RawPacket) -> KeisteenResult<Self> {
        Ok(Self)
    }

    fn handle(&self, conn: &mut Connection) -> KeisteenResult<()> {
        let (max, online, sample) = conn.server().read(|server| {
            let player_list = server.player_list();
            let max = player_list.max_players() as i32;
            let online = player_list.online_players() as i32;
            let sample = if online > 0 {
                let players = player_list.players().iter().take(12);
                let sample = players
                    .map(|p| StatusResponsePlayerSample {
                        name: p.profile().username().to_string(),
                        id: p.profile().uuid(),
                    })
                    .collect();
                Some(sample)
            } else {
                None
            };

            (max, online, sample)
        });

        let json_response = serde_json::to_string(&StatusResponse {
            version: StatusResponseVersion {
                name: crate::MC_VERSION.to_string(),
                protocol: Some(crate::MC_PROTOCOL.raw()),
            },
            players: Some(StatusResponsePlayers { max, online, sample }),
            description: Some(TextComponent {
                text: Some("A Minecraft Keisteen Server".to_string()),
                translate: None,
                color: None,
            }),
            favicon: None,
            enforces_secure_chat: false,
        })
        .expect("should serialize status response");

        conn.send_packet(client::status::StatusResponse { json_response })?;

        Ok(())
    }
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct StatusResponse {
    version: StatusResponseVersion,
    #[serde(skip_serializing_if = "Option::is_none")]
    players: Option<StatusResponsePlayers>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<TextComponent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    favicon: Option<String>,
    enforces_secure_chat: bool,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct StatusResponseVersion {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<i32>,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct StatusResponsePlayers {
    max: i32,
    online: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    sample: Option<Vec<StatusResponsePlayerSample>>,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct StatusResponsePlayerSample {
    name: String,
    id: Uuid,
}

#[derive(Debug)]
pub struct PingRequest {
    timestamp: i64,
}

impl ServerboundPacket for PingRequest {
    const PACKET_ID: i32 = 0x01;

    fn decode(mut raw: RawPacket) -> KeisteenResult<Self> {
        Ok(Self { timestamp: raw.data.read()? })
    }

    fn handle(&self, conn: &mut Connection) -> KeisteenResult<()> {
        conn.send_packet(client::status::PongResponse { timestamp: self.timestamp })?;
        conn.disconnect(None);
        Ok(())
    }
}
