use crate::protocol::packet::PacketData;
use crate::protocol::packet::client::ClientboundPacket;
use crate::types::VarInt;

#[derive(Debug)]
pub struct SetCompression {
    pub threshold: VarInt,
}

impl ClientboundPacket for SetCompression {
    const PACKET_ID: i32 = 0x03;

    fn encode(self, data: &mut PacketData) {
        data.write(self.threshold);
    }
}
