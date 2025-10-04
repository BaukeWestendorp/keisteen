use crate::protocol::packet::PacketData;
use crate::protocol::packet::client::ClientboundPacket;
use crate::types::Identifier;

#[derive(Debug)]
pub struct PluginMessage {
    pub channel: Identifier,
    pub data: Vec<u8>,
}

impl ClientboundPacket for PluginMessage {
    const PACKET_ID: i32 = 0x01;

    fn encode(self, data: &mut PacketData) {
        data.write(self.channel);
        data.write(self.data);
    }
}
