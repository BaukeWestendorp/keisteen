use crate::error::KeisteenResult;
use crate::mc::protocol::packet::{RawPacket, server};
use crate::server::conn::{Connection, ConnectionState};

pub mod decoder;
pub mod encoder;

impl Connection {
    pub fn handle_raw_packet(&mut self, raw: RawPacket) -> KeisteenResult<()> {
        match self.state {
            ConnectionState::Handshaking => server::handshake::handle_raw_packet(raw, self)?,
            ConnectionState::Status => server::status::handle_raw_packet(raw, self)?,
            ConnectionState::Transfer => todo!(),
            ConnectionState::Login => server::login::handle_raw_packet(raw, self)?,
            ConnectionState::Config => server::config::handle_raw_packet(raw, self)?,
            ConnectionState::Play => server::play::handle_raw_packet(raw, self)?,
        }

        Ok(())
    }
}
