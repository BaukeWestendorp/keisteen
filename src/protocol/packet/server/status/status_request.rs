use uuid::Uuid;

use crate::error::KeisteenResult;
use crate::protocol::packet::server::ServerboundPacket;
use crate::protocol::packet::{RawPacket, client};
use crate::server::conn::Connection;
use crate::text::text_component::TextComponent;

#[derive(Debug)]
pub struct StatusRequest;

impl ServerboundPacket for StatusRequest {
    const PACKET_ID: i32 = 0x00;

    fn decode(_raw: RawPacket) -> KeisteenResult<Self> {
        Ok(Self)
    }

    fn handle(&self, conn: &mut Connection) -> KeisteenResult<()> {
        let json_response = serde_json::to_string(&StatusResponse {
            version: StatusResponseVersion {
                name: crate::MC_VERSION.to_string(),
                protocol: Some(crate::MC_PROTOCOL.raw()),
            },
            players: Some(StatusResponsePlayers { max: 20, online: 0, sample: None }),
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
