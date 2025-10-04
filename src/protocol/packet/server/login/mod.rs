use crate::error::KeisteenResult;
use crate::protocol::packet::RawPacket;
use crate::protocol::packet::server::ServerboundPacket;
use crate::server::conn::Connection;

pub use cookie_response::*;
pub use encryption_response::*;
use eyre::bail;
pub use login_acknowledged::*;
pub use login_plugin_response::*;
pub use login_start::*;

mod cookie_response;
mod encryption_response;
mod login_acknowledged;
mod login_plugin_response;
mod login_start;

pub fn handle_raw_packet(raw: RawPacket, conn: &mut Connection) -> KeisteenResult<()> {
    match raw.packet_id.raw() {
        CookieResponse::PACKET_ID => CookieResponse::decode(raw)?.handle(conn),
        EncryptionResponse::PACKET_ID => EncryptionResponse::decode(raw)?.handle(conn),
        LoginAcknowledged::PACKET_ID => LoginAcknowledged::decode(raw)?.handle(conn),
        LoginPluginResponse::PACKET_ID => LoginPluginResponse::decode(raw)?.handle(conn),
        LoginStart::PACKET_ID => LoginStart::decode(raw)?.handle(conn),
        _ => bail!("unknown login packet id: {}", raw.packet_id.raw()),
    }
}
