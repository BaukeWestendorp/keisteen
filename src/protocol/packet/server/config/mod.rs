use crate::error::KeisteenResult;
use crate::protocol::packet::RawPacket;
use crate::protocol::packet::server::ServerboundPacket;
use crate::server::conn::Connection;

pub use acknowledge_finish_config::*;
pub use client_information::*;
pub use cookie_response::*;
pub use custom_click_action::*;
use eyre::bail;
pub use keep_alive::*;
pub use known_packs::*;
pub use plugin_message::*;
pub use pong::*;
pub use resource_pack_response::*;

mod acknowledge_finish_config;
mod client_information;
mod cookie_response;
mod custom_click_action;
mod keep_alive;
mod known_packs;
mod plugin_message;
mod pong;
mod resource_pack_response;

pub fn handle_raw_packet(raw: RawPacket, conn: &mut Connection) -> KeisteenResult<()> {
    match raw.packet_id.raw() {
        AcknowledgeFinishConfig::PACKET_ID => AcknowledgeFinishConfig::decode(raw)?.handle(conn),
        ClientInformation::PACKET_ID => ClientInformation::decode(raw)?.handle(conn),
        CookieResponse::PACKET_ID => CookieResponse::decode(raw)?.handle(conn),
        CustomClickAction::PACKET_ID => CustomClickAction::decode(raw)?.handle(conn),
        KeepAlive::PACKET_ID => KeepAlive::decode(raw)?.handle(conn),
        KnownPacks::PACKET_ID => KnownPacks::decode(raw)?.handle(conn),
        PluginMessage::PACKET_ID => PluginMessage::decode(raw)?.handle(conn),
        Pong::PACKET_ID => Pong::decode(raw)?.handle(conn),
        ResourcePackResponse::PACKET_ID => ResourcePackResponse::decode(raw)?.handle(conn),
        _ => bail!("unknown config packet id: {}", raw.packet_id.raw()),
    }
}
