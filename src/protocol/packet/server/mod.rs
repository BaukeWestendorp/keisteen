use crate::error::KeisteenResult;
use crate::protocol::packet::RawPacket;
use crate::server::conn::Connection;

pub mod config;
pub mod handshake;
pub mod login;
pub mod play;
pub mod status;

pub trait ServerboundPacket {
    const PACKET_ID: i32 = 0x00;

    fn decode(raw: RawPacket) -> KeisteenResult<Self>
    where
        Self: Sized;

    fn handle(&self, _conn: &mut Connection) -> KeisteenResult<()>;
}
