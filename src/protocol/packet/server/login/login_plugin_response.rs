use crate::error::KeisteenResult;
use crate::protocol::packet::RawPacket;
use crate::protocol::packet::server::ServerboundPacket;
use crate::server::conn::Connection;
use crate::types::VarInt;

pub struct LoginPluginResponse {
    pub message_id: VarInt,
    pub data: Vec<u8>,
}

impl ServerboundPacket for LoginPluginResponse {
    const PACKET_ID: i32 = 0x02;

    fn decode(_raw: RawPacket) -> KeisteenResult<Self> {
        todo!()
    }

    fn handle(&self, _conn: &mut Connection) -> KeisteenResult<()> {
        todo!()
    }
}
