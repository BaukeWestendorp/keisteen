use crate::error::KeisteenResult;
use crate::protocol::packet::SHandshakingPacket;
use crate::server::conn::Connection;

impl Connection {
    pub fn handle_handshaking_packet(&mut self, packet: SHandshakingPacket) -> KeisteenResult<()> {
        match packet {
            SHandshakingPacket::Handshake { intent, protocol_version, .. } => {
                if protocol_version != crate::MC_PROTOCOL {
                    log::warn!(
                        "client has protocol version {}, but server is {}",
                        protocol_version,
                        crate::MC_PROTOCOL
                    );

                    // TODO: Kick player.
                }

                self.state = intent;
            }
        }

        Ok(())
    }
}
