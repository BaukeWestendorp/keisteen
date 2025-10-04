use crate::error::KeisteenResult;
use crate::protocol::packet::RawPacket;
use crate::protocol::packet::server::ServerboundPacket;
use crate::server::conn::Connection;
use crate::types::VarInt;

#[derive(Debug)]
pub struct ClientInformation {
    pub locale: String,
    pub view_distance: i8,
    pub chat_mode: VarInt,
    pub chat_colors: bool,
    pub displayed_skin_parts: u8,
    pub main_hand: VarInt,
    pub enable_text_filtering: bool,
    pub allow_server_listing: bool,
    pub particle_status: VarInt,
}

impl ServerboundPacket for ClientInformation {
    const PACKET_ID: i32 = 0x00;

    fn decode(mut raw: RawPacket) -> KeisteenResult<Self> {
        Ok(Self {
            locale: raw.data.read()?,
            view_distance: raw.data.read()?,
            chat_mode: raw.data.read()?,
            chat_colors: raw.data.read()?,
            displayed_skin_parts: raw.data.read()?,
            main_hand: raw.data.read()?,
            enable_text_filtering: raw.data.read()?,
            allow_server_listing: raw.data.read()?,
            particle_status: raw.data.read()?,
        })
    }

    fn handle(&self, _conn: &mut Connection) -> KeisteenResult<()> {
        // TODO: Do something with client information.
        Ok(())
    }
}
