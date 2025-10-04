use crate::protocol::packet::PacketData;
use crate::protocol::packet::client::ClientboundPacket;
use crate::protocol::packet::known_pack::KnownPack;
use crate::protocol::packet::registry_data_entry::RegistryDataEntry;
use crate::types::Identifier;

#[derive(Debug)]
pub struct CookieRequest;

impl ClientboundPacket for CookieRequest {
    const PACKET_ID: i32 = 0x00;

    fn encode(self, _data: &mut PacketData) {
        todo!()
    }
}

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

#[derive(Debug)]
pub struct Disconnected;

impl ClientboundPacket for Disconnected {
    const PACKET_ID: i32 = 0x02;

    fn encode(self, _data: &mut PacketData) {
        todo!()
    }
}

#[derive(Debug)]
pub struct FinishConfig;

impl ClientboundPacket for FinishConfig {
    const PACKET_ID: i32 = 0x03;

    fn encode(self, _data: &mut PacketData) {}
}

#[derive(Debug)]
pub struct KeepAlive;

impl ClientboundPacket for KeepAlive {
    const PACKET_ID: i32 = 0x04;

    fn encode(self, _data: &mut PacketData) {
        todo!()
    }
}

#[derive(Debug)]
pub struct Ping;

impl ClientboundPacket for Ping {
    const PACKET_ID: i32 = 0x05;

    fn encode(self, _data: &mut PacketData) {
        todo!()
    }
}

#[derive(Debug)]
pub struct ResetChat;

impl ClientboundPacket for ResetChat {
    const PACKET_ID: i32 = 0x06;

    fn encode(self, _data: &mut PacketData) {
        todo!()
    }
}

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

#[derive(Debug)]
pub struct RemoveResourcePack;

impl ClientboundPacket for RemoveResourcePack {
    const PACKET_ID: i32 = 0x08;

    fn encode(self, _data: &mut PacketData) {
        todo!()
    }
}

#[derive(Debug)]
pub struct AddResourcePack;

impl ClientboundPacket for AddResourcePack {
    const PACKET_ID: i32 = 0x09;

    fn encode(self, _data: &mut PacketData) {
        todo!()
    }
}

#[derive(Debug)]
pub struct StoreCookie;

impl ClientboundPacket for StoreCookie {
    const PACKET_ID: i32 = 0x0A;

    fn encode(self, _data: &mut PacketData) {
        todo!()
    }
}

#[derive(Debug)]
pub struct Transfer;

impl ClientboundPacket for Transfer {
    const PACKET_ID: i32 = 0x0B;

    fn encode(self, _data: &mut PacketData) {
        todo!()
    }
}

#[derive(Debug)]
pub struct FeatureFlags;

impl ClientboundPacket for FeatureFlags {
    const PACKET_ID: i32 = 0x0C;

    fn encode(self, _data: &mut PacketData) {
        todo!()
    }
}

#[derive(Debug)]
pub struct UpdateTags;

impl ClientboundPacket for UpdateTags {
    const PACKET_ID: i32 = 0x0D;

    fn encode(self, _data: &mut PacketData) {
        todo!()
    }
}

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

#[derive(Debug)]
pub struct CustomReportDetails;

impl ClientboundPacket for CustomReportDetails {
    const PACKET_ID: i32 = 0x0F;

    fn encode(self, _data: &mut PacketData) {
        todo!()
    }
}

#[derive(Debug)]
pub struct ServerLinks;

impl ClientboundPacket for ServerLinks {
    const PACKET_ID: i32 = 0x10;

    fn encode(self, _data: &mut PacketData) {
        todo!()
    }
}

#[derive(Debug)]
pub struct ClearDialog;

impl ClientboundPacket for ClearDialog {
    const PACKET_ID: i32 = 0x11;

    fn encode(self, _data: &mut PacketData) {
        todo!()
    }
}

#[derive(Debug)]
pub struct ShowDialog;

impl ClientboundPacket for ShowDialog {
    const PACKET_ID: i32 = 0x12;

    fn encode(self, _data: &mut PacketData) {
        todo!()
    }
}
