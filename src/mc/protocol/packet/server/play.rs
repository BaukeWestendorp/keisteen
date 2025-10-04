use crate::error::KeisteenResult;
use crate::mc::protocol::packet::RawPacket;
use crate::server::conn::Connection;

pub fn handle_raw_packet(raw: RawPacket, _conn: &mut Connection) -> KeisteenResult<()> {
    match raw.packet_id.raw() {
        _ => Ok(()),
    }
}
