use crate::error::CraftError;
use crate::server::packet::PacketData;
use crate::types::{Identifier, VarInt};

use super::RawPacket;

#[derive(Debug)]
pub enum CConfigurationPacket {
    CookieRequest,
    PluginMessage,
    Disconnected,
    FinishConfiguration,
    KeepAlive,
    Ping,
    ResetChat,
    RegistryData,
    RemoveResourcePack,
    AddResourcePack,
    StoreCookie,
    Transfer,
    FeatureFlags,
    UpdateTags,
    KnownPacks,
    CustomReportDetails,
    ServerLinks,
    ClearDialog,
    ShowDialog,
}

impl From<CConfigurationPacket> for RawPacket {
    fn from(packet: CConfigurationPacket) -> Self {
        match packet {
            CConfigurationPacket::CookieRequest => todo!(),
            CConfigurationPacket::PluginMessage => todo!(),
            CConfigurationPacket::Disconnected => todo!(),
            CConfigurationPacket::FinishConfiguration => {
                RawPacket { packet_id: VarInt::new(0x03), data: PacketData::new() }
            }
            CConfigurationPacket::KeepAlive => todo!(),
            CConfigurationPacket::Ping => todo!(),
            CConfigurationPacket::ResetChat => todo!(),
            CConfigurationPacket::RegistryData => todo!(),
            CConfigurationPacket::RemoveResourcePack => todo!(),
            CConfigurationPacket::AddResourcePack => todo!(),
            CConfigurationPacket::StoreCookie => todo!(),
            CConfigurationPacket::Transfer => todo!(),
            CConfigurationPacket::FeatureFlags => todo!(),
            CConfigurationPacket::UpdateTags => todo!(),
            CConfigurationPacket::KnownPacks => todo!(),
            CConfigurationPacket::CustomReportDetails => todo!(),
            CConfigurationPacket::ServerLinks => todo!(),
            CConfigurationPacket::ClearDialog => todo!(),
            CConfigurationPacket::ShowDialog => todo!(),
        }
    }
}

#[derive(Debug)]
pub enum SConfigurationPacket {
    ClientInformation {
        locale: String,
        view_distance: i8,
        chat_mode: VarInt,
        chat_colors: bool,
        displayed_skin_parts: u8,
        main_hand: VarInt,
        enable_text_filtering: bool,
        allow_server_listing: bool,
        particle_status: VarInt,
    },
    CookieResponse,
    PluginMessage {
        channel: Identifier,
        data: Vec<u8>,
    },
    AcknowledgeFinishConfiguration,
    KeepAlive,
    Pong,
    ResourcePackResponse,
    KnownPacks,
    CustomClickAction,
}

impl TryFrom<RawPacket> for SConfigurationPacket {
    type Error = CraftError;

    fn try_from(mut packet: RawPacket) -> Result<Self, Self::Error> {
        match packet.packet_id.raw() {
            0x00 => Ok(Self::ClientInformation {
                locale: packet.data.consume_string(16)?,
                view_distance: packet.data.consume_i8()?,
                chat_mode: packet.data.consume_varint()?,
                chat_colors: packet.data.consume_bool()?,
                displayed_skin_parts: packet.data.consume_u8()?,
                main_hand: packet.data.consume_varint()?,
                enable_text_filtering: packet.data.consume_bool()?,
                allow_server_listing: packet.data.consume_bool()?,
                particle_status: packet.data.consume_varint()?,
            }),
            0x01 => todo!(), // Ok(Self::CookieResponse),
            0x02 => {
                let channel = packet.data.consume_identifier()?;
                let data_len = packet.data.bytes().len();
                Ok(Self::PluginMessage { channel, data: packet.data.consume_byte_array(data_len)? })
            }
            0x03 => Ok(Self::AcknowledgeFinishConfiguration),
            0x04 => todo!(), // Ok(Self::KeepAlive),
            0x05 => todo!(), // Ok(Self::Pong),
            0x06 => todo!(), // Ok(Self::ResourcePackResponse),
            0x07 => todo!(), // Ok(Self::KnownPacks),
            0x08 => todo!(), // Ok(Self::CustomClickAction),
            _ => return Err(CraftError::InvalidPacket),
        }
    }
}
