use bytes::Bytes;

use crate::error::KeisteenResult;
use crate::mc::packet::ServerboundRawPacket;
use crate::mc::packet::server::ServerboundPacket;
use crate::server::connection::Connection;

#[derive(Debug)]
pub struct ClientInformation;

impl ServerboundPacket for ClientInformation {
    const PACKET_ID: i32 = 0x00;

    fn decode_data(_bytes: Bytes) -> KeisteenResult<Self> {
        todo!()
    }

    async fn handle(self, _connection: &mut Connection) -> KeisteenResult<()> {
        todo!()
    }
}

#[derive(Debug)]
pub struct CookieResponse;

impl ServerboundPacket for CookieResponse {
    const PACKET_ID: i32 = 0x01;

    fn decode_data(_bytes: Bytes) -> KeisteenResult<Self> {
        todo!()
    }

    async fn handle(self, _connection: &mut Connection) -> KeisteenResult<()> {
        todo!()
    }
}

#[derive(Debug)]
pub struct PluginMessage;

impl ServerboundPacket for PluginMessage {
    const PACKET_ID: i32 = 0x02;

    fn decode_data(_bytes: Bytes) -> KeisteenResult<Self> {
        todo!()
    }

    async fn handle(self, _connection: &mut Connection) -> KeisteenResult<()> {
        todo!()
    }
}

#[derive(Debug)]
pub struct AcknowledgeFinishConfiguration;

impl ServerboundPacket for AcknowledgeFinishConfiguration {
    const PACKET_ID: i32 = 0x03;

    fn decode_data(_bytes: Bytes) -> KeisteenResult<Self> {
        todo!()
    }

    async fn handle(self, _connection: &mut Connection) -> KeisteenResult<()> {
        todo!()
    }
}

#[derive(Debug)]
pub struct KeepAlive;

impl ServerboundPacket for KeepAlive {
    const PACKET_ID: i32 = 0x04;

    fn decode_data(_bytes: Bytes) -> KeisteenResult<Self> {
        todo!()
    }

    async fn handle(self, _connection: &mut Connection) -> KeisteenResult<()> {
        todo!()
    }
}

#[derive(Debug)]
pub struct Pong;

impl ServerboundPacket for Pong {
    const PACKET_ID: i32 = 0x05;

    fn decode_data(_bytes: Bytes) -> KeisteenResult<Self> {
        todo!()
    }

    async fn handle(self, _connection: &mut Connection) -> KeisteenResult<()> {
        todo!()
    }
}

#[derive(Debug)]
pub struct ResourcePackResponse;

impl ServerboundPacket for ResourcePackResponse {
    const PACKET_ID: i32 = 0x06;

    fn decode_data(_bytes: Bytes) -> KeisteenResult<Self> {
        todo!()
    }

    async fn handle(self, _connection: &mut Connection) -> KeisteenResult<()> {
        todo!()
    }
}

#[derive(Debug)]
pub struct KnownPacks;

impl ServerboundPacket for KnownPacks {
    const PACKET_ID: i32 = 0x07;

    fn decode_data(_bytes: Bytes) -> KeisteenResult<Self> {
        todo!()
    }

    async fn handle(self, _connection: &mut Connection) -> KeisteenResult<()> {
        todo!()
    }
}

#[derive(Debug)]
pub struct CustomClickAction;

impl ServerboundPacket for CustomClickAction {
    const PACKET_ID: i32 = 0x08;

    fn decode_data(_bytes: Bytes) -> KeisteenResult<Self> {
        todo!()
    }

    async fn handle(self, _connection: &mut Connection) -> KeisteenResult<()> {
        todo!()
    }
}

pub async fn handle_raw_packet(
    packet: ServerboundRawPacket,
    connection: &mut Connection,
) -> KeisteenResult<()> {
    match packet.id.raw() {
        ClientInformation::PACKET_ID => {
            ClientInformation::decode_data(packet.data)?.handle(connection).await?;
        }
        CookieResponse::PACKET_ID => {
            CookieResponse::decode_data(packet.data)?.handle(connection).await?;
        }
        PluginMessage::PACKET_ID => {
            PluginMessage::decode_data(packet.data)?.handle(connection).await?;
        }
        AcknowledgeFinishConfiguration::PACKET_ID => {
            AcknowledgeFinishConfiguration::decode_data(packet.data)?.handle(connection).await?;
        }
        KeepAlive::PACKET_ID => {
            KeepAlive::decode_data(packet.data)?.handle(connection).await?;
        }
        Pong::PACKET_ID => {
            Pong::decode_data(packet.data)?.handle(connection).await?;
        }
        ResourcePackResponse::PACKET_ID => {
            ResourcePackResponse::decode_data(packet.data)?.handle(connection).await?;
        }
        KnownPacks::PACKET_ID => {
            KnownPacks::decode_data(packet.data)?.handle(connection).await?;
        }
        CustomClickAction::PACKET_ID => {
            CustomClickAction::decode_data(packet.data)?.handle(connection).await?;
        }
        _ => {
            log::warn!("unknown packet id: {}", packet.id.raw());
        }
    }

    Ok(())
}
