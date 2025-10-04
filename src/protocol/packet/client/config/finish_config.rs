use crate::protocol::packet::PacketData;
use crate::protocol::packet::client::ClientboundPacket;

#[derive(Debug)]
pub struct FinishConfig;

impl ClientboundPacket for FinishConfig {
    const PACKET_ID: i32 = 0x03;

    fn encode(self, _data: &mut PacketData) {}
}
