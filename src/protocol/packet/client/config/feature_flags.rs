use crate::protocol::packet::PacketData;
use crate::protocol::packet::client::ClientboundPacket;

#[derive(Debug)]
pub struct FeatureFlags;

impl ClientboundPacket for FeatureFlags {
    const PACKET_ID: i32 = 0x0C;

    fn encode(self, _data: &mut PacketData) {
        todo!()
    }
}
