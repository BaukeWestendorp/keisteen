use crate::protocol::packet::{CStatusPacket, SStatusPacket};
use crate::server::conn::Connection;

impl Connection {
    pub fn handle_status_packet(&mut self, packet: SStatusPacket) -> crate::error::Result<()> {
        match packet {
            SStatusPacket::StatusRequest => {
                // TODO: Populate this JSON with actual data.
                let json_response = r#"{"version":{"name":"1.21.8","protocol":772},"players":{"max":20,"online":0,"sample":[]},"description":{"text":"\u00a74!!\u00a76\u00a7l craft\u00a74 !!"},"enforcesSecureChat":false}"#.to_string();
                self.send_packet(CStatusPacket::StatusResponse { json_response })?;
            }
            SStatusPacket::PingRequest { timestamp } => {
                self.send_packet(CStatusPacket::PongResponse { timestamp })?;
                self.close()?;
            }
        }

        Ok(())
    }
}
