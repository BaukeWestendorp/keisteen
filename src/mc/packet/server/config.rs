use bytes::{Buf, Bytes};

use crate::error::KeisteenResult;
use crate::mc::packet::server::ServerboundPacket;
use crate::mc::packet::{KnownPack, ServerboundRawPacket};
use crate::mc::protocol::BytesExt;
use crate::mc::types::{Identifier, VarInt};
use crate::server::connection::Connection;

#[derive(Debug)]
pub struct ClientInformation {
    pub locale: String,
    pub view_distance: u8,
    pub chat_mode: VarInt,
    pub chat_colors: bool,
    pub displayed_skin_parts: u8,
    pub main_hand: VarInt,
    pub enable_text_filtering: bool,
    pub allow_server_listings: bool,
    pub particle_status: VarInt,
}

impl ServerboundPacket for ClientInformation {
    const PACKET_ID: i32 = 0x00;

    fn decode_data(mut bytes: Bytes) -> KeisteenResult<Self> {
        Ok(Self {
            locale: bytes.try_get_prefixed_string()?,
            view_distance: bytes.try_get_u8()?,
            chat_mode: bytes.try_get_varint()?,
            chat_colors: bytes.try_get_bool()?,
            displayed_skin_parts: bytes.try_get_u8()?,
            main_hand: bytes.try_get_varint()?,
            enable_text_filtering: bytes.try_get_bool()?,
            allow_server_listings: bytes.try_get_bool()?,
            particle_status: bytes.try_get_varint()?,
        })
    }

    async fn handle(self, _connection: &mut Connection) -> KeisteenResult<()> {
        log::trace!("<<< {self:?}");

        Ok(())
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
pub struct PluginMessage {
    pub channel: Identifier,
    pub data: Bytes,
}

impl ServerboundPacket for PluginMessage {
    const PACKET_ID: i32 = 0x02;

    fn decode_data(mut bytes: Bytes) -> KeisteenResult<Self> {
        Ok(Self { channel: bytes.try_get_identifier()?, data: bytes.split_to(bytes.len()) })
    }

    async fn handle(self, _connection: &mut Connection) -> KeisteenResult<()> {
        log::trace!("<<< {self:?}");

        Ok(())
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
pub struct KnownPacks {
    pub known_packs: Vec<KnownPack>,
}

impl ServerboundPacket for KnownPacks {
    const PACKET_ID: i32 = 0x07;

    fn decode_data(mut bytes: Bytes) -> KeisteenResult<Self> {
        Ok(Self {
            known_packs: bytes.try_get_prefixed_array(|bytes| {
                Ok(KnownPack {
                    namespace: bytes.try_get_prefixed_string()?,
                    id: bytes.try_get_prefixed_string()?,
                    version: bytes.try_get_prefixed_string()?,
                })
            })?,
        })
    }

    async fn handle(self, _connection: &mut Connection) -> KeisteenResult<()> {
        log::trace!("<<< {self:?}");

        Ok(())
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
