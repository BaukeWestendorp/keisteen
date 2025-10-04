use crate::error::KeisteenResult;
use crate::protocol::packet::server::ServerboundPacket;
use crate::protocol::packet::{RawPacket, client};
use crate::server::conn::Connection;

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
        conn.close()?;
        Ok(())
    }
}
