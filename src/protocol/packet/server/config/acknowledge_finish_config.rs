use crate::error::KeisteenResult;
use crate::protocol::packet::RawPacket;
use crate::protocol::packet::server::ServerboundPacket;
use crate::server::conn::{Connection, ConnectionState};

#[derive(Debug)]
pub struct AcknowledgeFinishConfig;

impl ServerboundPacket for AcknowledgeFinishConfig {
    const PACKET_ID: i32 = 0x03;

    fn decode(_raw: RawPacket) -> KeisteenResult<Self> {
        Ok(Self)
    }

    fn handle(&self, conn: &mut Connection) -> KeisteenResult<()> {
        log::debug!("configuration acknowledged");
        conn.state = ConnectionState::Play;
        Ok(())
    }
}
