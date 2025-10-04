use crate::error::KeisteenResult;
use crate::protocol::packet::RawPacket;
use crate::protocol::packet::server::ServerboundPacket;
use crate::server::conn::Connection;
use crate::types::Identifier;

#[derive(Debug)]
pub struct CookieResponse {
    pub key: Identifier,
    pub payload: Vec<u8>,
}

impl ServerboundPacket for CookieResponse {
    const PACKET_ID: i32 = 0x04;

    fn decode(_raw: RawPacket) -> KeisteenResult<Self> {
        todo!()
    }

    fn handle(&self, _conn: &mut Connection) -> KeisteenResult<()> {
        todo!()
    }
}
