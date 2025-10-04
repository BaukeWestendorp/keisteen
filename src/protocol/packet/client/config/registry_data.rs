use crate::protocol::packet::PacketData;
use crate::protocol::packet::client::ClientboundPacket;
use crate::protocol::packet::registry_data_entry::RegistryDataEntry;
use crate::types::Identifier;

#[derive(Debug)]
pub struct RegistryData {
    pub registry_id: Identifier,
    pub entries: Vec<RegistryDataEntry>,
}

impl ClientboundPacket for RegistryData {
    const PACKET_ID: i32 = 0x07;

    fn encode(self, data: &mut PacketData) {
        data.write(self.registry_id);
        data.write_prefixed(self.entries);
    }
}
