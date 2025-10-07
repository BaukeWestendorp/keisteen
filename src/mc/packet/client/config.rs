use bytes::BytesMut;

use crate::mc::packet::client::ClientboundPacket;

#[derive(Debug)]
pub struct CookieRequest;

impl ClientboundPacket for CookieRequest {
    const PACKET_ID: i32 = 0x01;

    fn encode_data(self, _bytes: &mut BytesMut) {
        todo!()
    }
}

#[derive(Debug)]
pub struct PluginMessage;

impl ClientboundPacket for PluginMessage {
    const PACKET_ID: i32 = 0x02;

    fn encode_data(self, _bytes: &mut BytesMut) {
        todo!()
    }
}

#[derive(Debug)]
pub struct Disconnect;

impl ClientboundPacket for Disconnect {
    const PACKET_ID: i32 = 0x03;

    fn encode_data(self, _bytes: &mut BytesMut) {
        todo!()
    }
}

#[derive(Debug)]
pub struct FinishConfiguration;

impl ClientboundPacket for FinishConfiguration {
    const PACKET_ID: i32 = 0x04;

    fn encode_data(self, _bytes: &mut BytesMut) {
        todo!()
    }
}

#[derive(Debug)]
pub struct KeepAlive;

impl ClientboundPacket for KeepAlive {
    const PACKET_ID: i32 = 0x05;

    fn encode_data(self, _bytes: &mut BytesMut) {
        todo!()
    }
}

#[derive(Debug)]
pub struct Ping;

impl ClientboundPacket for Ping {
    const PACKET_ID: i32 = 0x06;

    fn encode_data(self, _bytes: &mut BytesMut) {
        todo!()
    }
}

#[derive(Debug)]
pub struct ResetChat;

impl ClientboundPacket for ResetChat {
    const PACKET_ID: i32 = 0x07;

    fn encode_data(self, _bytes: &mut BytesMut) {
        todo!()
    }
}

#[derive(Debug)]
pub struct RegistryData;

impl ClientboundPacket for RegistryData {
    const PACKET_ID: i32 = 0x08;

    fn encode_data(self, _bytes: &mut BytesMut) {
        todo!()
    }
}

#[derive(Debug)]
pub struct RemoveResourcePack;

impl ClientboundPacket for RemoveResourcePack {
    const PACKET_ID: i32 = 0x09;

    fn encode_data(self, _bytes: &mut BytesMut) {
        todo!()
    }
}

#[derive(Debug)]
pub struct AddResourcePack;

impl ClientboundPacket for AddResourcePack {
    const PACKET_ID: i32 = 0x0A;

    fn encode_data(self, _bytes: &mut BytesMut) {
        todo!()
    }
}

#[derive(Debug)]
pub struct StoreCookie;

impl ClientboundPacket for StoreCookie {
    const PACKET_ID: i32 = 0x0B;

    fn encode_data(self, _bytes: &mut BytesMut) {
        todo!()
    }
}

#[derive(Debug)]
pub struct Transfer;

impl ClientboundPacket for Transfer {
    const PACKET_ID: i32 = 0x0C;

    fn encode_data(self, _bytes: &mut BytesMut) {
        todo!()
    }
}

#[derive(Debug)]
pub struct FeatureFlags;

impl ClientboundPacket for FeatureFlags {
    const PACKET_ID: i32 = 0x0D;

    fn encode_data(self, _bytes: &mut BytesMut) {
        todo!()
    }
}

#[derive(Debug)]
pub struct UpdateTags;

impl ClientboundPacket for UpdateTags {
    const PACKET_ID: i32 = 0x0E;

    fn encode_data(self, _bytes: &mut BytesMut) {
        todo!()
    }
}

#[derive(Debug)]
pub struct KnownPacks;

impl ClientboundPacket for KnownPacks {
    const PACKET_ID: i32 = 0x0F;

    fn encode_data(self, _bytes: &mut BytesMut) {
        todo!()
    }
}

#[derive(Debug)]
pub struct CustomReportDetails;

impl ClientboundPacket for CustomReportDetails {
    const PACKET_ID: i32 = 0x10;

    fn encode_data(self, _bytes: &mut BytesMut) {
        todo!()
    }
}

#[derive(Debug)]
pub struct ServerLinks;

impl ClientboundPacket for ServerLinks {
    const PACKET_ID: i32 = 0x11;

    fn encode_data(self, _bytes: &mut BytesMut) {
        todo!()
    }
}

#[derive(Debug)]
pub struct ClearDialog;

impl ClientboundPacket for ClearDialog {
    const PACKET_ID: i32 = 0x12;

    fn encode_data(self, _bytes: &mut BytesMut) {
        todo!()
    }
}

#[derive(Debug)]
pub struct ShowDialog;

impl ClientboundPacket for ShowDialog {
    const PACKET_ID: i32 = 0x13;

    fn encode_data(self, _bytes: &mut BytesMut) {
        todo!()
    }
}
