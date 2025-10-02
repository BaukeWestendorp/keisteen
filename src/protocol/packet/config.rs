use std::io;

use eyre::bail;

use crate::nbt::CompoundTag;
use crate::protocol::packet::{PacketData, PrefixedProtocolWrite, ProtocolWrite};
use crate::types::{Identifier, VarInt};

use super::RawPacket;

#[derive(Debug)]
pub enum CConfigurationPacket {
    CookieRequest,
    PluginMessage { channel: Identifier, data: Vec<u8> },
    Disconnected,
    FinishConfiguration,
    KeepAlive,
    Ping,
    ResetChat,
    RegistryData { registry_id: Identifier, entries: Vec<RegistryDataEntry> },
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
            CConfigurationPacket::PluginMessage { channel, data } => RawPacket {
                packet_id: VarInt::new(0x01),
                data: {
                    let mut packet_data = PacketData::new();
                    packet_data.write_all(channel);
                    packet_data.write_all(data);
                    packet_data
                },
            },
            CConfigurationPacket::Disconnected => todo!(),
            CConfigurationPacket::FinishConfiguration => {
                RawPacket { packet_id: VarInt::new(0x03), data: PacketData::new() }
            }
            CConfigurationPacket::KeepAlive => todo!(),
            CConfigurationPacket::Ping => todo!(),
            CConfigurationPacket::ResetChat => todo!(),
            CConfigurationPacket::RegistryData { registry_id, entries } => RawPacket {
                packet_id: VarInt::new(0x07),
                data: {
                    let mut data = PacketData::new();
                    data.write_all(registry_id);
                    data.write_all(entries);
                    data
                },
            },
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
    type Error = crate::error::Error;

    fn try_from(mut packet: RawPacket) -> Result<Self, Self::Error> {
        match packet.packet_id.raw() {
            0x00 => Ok(Self::ClientInformation {
                locale: packet.data.read()?,
                view_distance: packet.data.read()?,
                chat_mode: packet.data.read()?,
                chat_colors: packet.data.read()?,
                displayed_skin_parts: packet.data.read()?,
                main_hand: packet.data.read()?,
                enable_text_filtering: packet.data.read()?,
                allow_server_listing: packet.data.read()?,
                particle_status: packet.data.read()?,
            }),
            0x01 => todo!(), // Ok(Self::CookieResponse),
            0x02 => {
                let channel = packet.data.read()?;
                let data_len = packet.data.bytes().len();
                Ok(Self::PluginMessage { channel, data: packet.data.read_predefined(data_len)? })
            }
            0x03 => Ok(Self::AcknowledgeFinishConfiguration),
            0x04 => todo!(), // Ok(Self::KeepAlive),
            0x05 => todo!(), // Ok(Self::Pong),
            0x06 => todo!(), // Ok(Self::ResourcePackResponse),
            0x07 => todo!(), // Ok(Self::KnownPacks),
            0x08 => todo!(), // Ok(Self::CustomClickAction),
            packet_id => bail!("invalid packet id: {packet_id:#04x}"),
        }
    }
}

#[derive(Debug)]
pub struct RegistryDataEntry {
    entry_id: Identifier,
    data: Option<CompoundTag>,
}

impl ProtocolWrite for RegistryDataEntry {
    fn write_all<W: io::Write>(&self, writer: &mut W) -> crate::error::Result<()> {
        self.entry_id.write_all(writer)?;
        self.data.prefixed_write_all(writer)?;
        Ok(())
    }
}
