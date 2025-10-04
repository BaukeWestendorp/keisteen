use crate::error::KeisteenResult;
use crate::protocol::packet::RawPacket;
use crate::protocol::packet::server::ServerboundPacket;
use crate::server::conn::Connection;

#[derive(Debug)]
pub struct CookieResponse;

impl ServerboundPacket for CookieResponse {
    const PACKET_ID: i32 = 0x01;

    fn decode(_raw: RawPacket) -> KeisteenResult<Self> {
        todo!()
    }

    fn handle(&self, _conn: &mut Connection) -> KeisteenResult<()> {
        todo!()
    }
}
