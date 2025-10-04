use crate::error::KeisteenResult;
use crate::protocol::packet::RawPacket;
use crate::protocol::packet::server::ServerboundPacket;
use crate::server::conn::Connection;

use eyre::bail;
pub use handshake::*;

mod handshake;

pub fn handle_raw_packet(raw: RawPacket, conn: &mut Connection) -> KeisteenResult<()> {
    match raw.packet_id.raw() {
        Handshake::PACKET_ID => Handshake::decode(raw)?.handle(conn),
        _ => bail!("unknown handshake packet id: {}", raw.packet_id.raw()),
    }
}
