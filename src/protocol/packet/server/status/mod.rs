use crate::error::KeisteenResult;
use crate::protocol::packet::RawPacket;
use crate::protocol::packet::server::ServerboundPacket;
use crate::server::conn::Connection;

use eyre::bail;

pub use ping_request::*;
pub use status_request::*;

mod ping_request;
mod status_request;

pub fn handle_raw_packet(raw: RawPacket, conn: &mut Connection) -> KeisteenResult<()> {
    match raw.packet_id.raw() {
        StatusRequest::PACKET_ID => StatusRequest::decode(raw)?.handle(conn),
        PingRequest::PACKET_ID => PingRequest::decode(raw)?.handle(conn),
        _ => bail!("unknown status packet id: {}", raw.packet_id.raw()),
    }
}
