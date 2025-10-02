use crate::protocol::packet::SHandshakingPacket;
use crate::server::conn::Connection;

impl Connection {
    pub fn handle_handshaking_packet(
        &mut self,
        packet: SHandshakingPacket,
    ) -> crate::error::Result<()> {
        match packet {
            SHandshakingPacket::Handshake { intent, .. } => {
                self.state = intent;
            }
        }

        Ok(())
    }
}
