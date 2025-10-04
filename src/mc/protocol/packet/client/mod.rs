use crate::mc::protocol::packet::PacketData;

pub mod config;
pub mod login;
pub mod play;
pub mod status;

pub trait ClientboundPacket {
    const PACKET_ID: i32 = 0x00;

    fn encode(self, data: &mut PacketData);
}
