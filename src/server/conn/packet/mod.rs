use crate::error::KeisteenResult;
use crate::protocol::packet::{
    RawPacket, SConfigPacket, SHandshakingPacket, SLoginPacket, SStatusPacket,
};
use crate::server::conn::{Connection, ConnectionState};

pub mod config;
pub mod handshaking;
pub mod login;
pub mod status;

impl Connection {
    pub fn handle_raw_packet(&mut self, packet: RawPacket) -> KeisteenResult<()> {
        match self.state {
            ConnectionState::Handshaking => {
                let packet = SHandshakingPacket::try_from(packet)?;
                self.handle_handshaking_packet(packet)?;
            }
            ConnectionState::Status => {
                let packet = SStatusPacket::try_from(packet)?;
                self.handle_status_packet(packet)?;
            }
            ConnectionState::Transfer => {
                todo!();
            }
            ConnectionState::Login => {
                let packet = SLoginPacket::try_from(packet)?;
                self.handle_login_packet(packet)?;
            }
            ConnectionState::Config => {
                let packet = SConfigPacket::try_from(packet)?;
                self.handle_config_packet(packet)?;
            }
            ConnectionState::Play => {
                todo!();
            }
        }

        Ok(())
    }
}
