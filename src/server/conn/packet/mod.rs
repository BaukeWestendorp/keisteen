use crate::error::KeisteenResult;
use crate::protocol::packet::{
    RawPacket, SConfigPacket, SHandshakingPacket, SLoginPacket, SStatusPacket, ServerboundPacket,
};
use crate::server::conn::{Connection, ConnectionState};

pub mod config;
pub mod handshaking;
pub mod login;
pub mod status;

impl Connection {
    pub fn handle_raw_packet(&mut self, raw: RawPacket) -> KeisteenResult<()> {
        match self.state {
            ConnectionState::Handshaking => {
                let packet = SHandshakingPacket::decode(raw)?;
                self.handle_handshaking_packet(packet)?;
            }
            ConnectionState::Status => {
                let packet = SStatusPacket::decode(raw)?;
                self.handle_status_packet(packet)?;
            }
            ConnectionState::Transfer => {
                todo!();
            }
            ConnectionState::Login => {
                let packet = SLoginPacket::decode(raw)?;
                self.handle_login_packet(packet)?;
            }
            ConnectionState::Config => {
                let packet = SConfigPacket::decode(raw)?;
                self.handle_config_packet(packet)?;
            }
            ConnectionState::Play => {
                todo!();
            }
        }

        Ok(())
    }
}
