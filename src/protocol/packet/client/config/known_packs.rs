use crate::protocol::packet::PacketData;
use crate::protocol::packet::client::ClientboundPacket;
use crate::protocol::packet::known_pack::KnownPack;

#[derive(Debug)]
pub struct KnownPacks {
    pub known_packs: Vec<KnownPack>,
}

impl ClientboundPacket for KnownPacks {
    const PACKET_ID: i32 = 0x0E;

    fn encode(self, data: &mut PacketData) {
        data.write_prefixed(self.known_packs);
    }
}
