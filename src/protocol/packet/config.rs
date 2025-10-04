use std::io;

use crate::error::KeisteenResult;
use crate::nbt;
use crate::protocol::packet::{
    PacketData, PrefixedProtocolWrite, ProtocolRead, ProtocolWrite, ServerboundPacket,
};
use crate::types::{Identifier, VarInt};

use super::RawPacket;

#[derive(Debug)]
pub enum CConfigPacket {
    CookieRequest,
    PluginMessage { channel: Identifier, data: Vec<u8> },
    Disconnected,
    FinishConfig,
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
    KnownPacks { known_packs: Vec<KnownPack> },
    CustomReportDetails,
    ServerLinks,
    ClearDialog,
    ShowDialog,
}

impl From<CConfigPacket> for RawPacket {
    fn from(packet: CConfigPacket) -> Self {
        match packet {
            CConfigPacket::CookieRequest => todo!(),
            CConfigPacket::PluginMessage { channel, data } => RawPacket {
                packet_id: VarInt::new(0x01),
                data: {
                    let mut packet_data = PacketData::new();
                    packet_data.write_all(channel);
                    packet_data.write_all(data);
                    packet_data
                },
            },
            CConfigPacket::Disconnected => todo!(),
            CConfigPacket::FinishConfig => {
                RawPacket { packet_id: VarInt::new(0x03), data: PacketData::new() }
            }
            CConfigPacket::KeepAlive => todo!(),
            CConfigPacket::Ping => todo!(),
            CConfigPacket::ResetChat => todo!(),
            CConfigPacket::RegistryData { registry_id, entries } => RawPacket {
                packet_id: VarInt::new(0x07),
                data: {
                    let mut data = PacketData::new();
                    data.write_all(registry_id);
                    data.write_all_prefixed(entries);
                    data
                },
            },
            CConfigPacket::RemoveResourcePack => todo!(),
            CConfigPacket::AddResourcePack => todo!(),
            CConfigPacket::StoreCookie => todo!(),
            CConfigPacket::Transfer => todo!(),
            CConfigPacket::FeatureFlags => todo!(),
            CConfigPacket::UpdateTags => todo!(),
            CConfigPacket::KnownPacks { known_packs } => RawPacket {
                packet_id: VarInt::new(0xE),
                data: {
                    let mut data = PacketData::new();
                    data.write_all_prefixed(known_packs);
                    data
                },
            },
            CConfigPacket::CustomReportDetails => todo!(),
            CConfigPacket::ServerLinks => todo!(),
            CConfigPacket::ClearDialog => todo!(),
            CConfigPacket::ShowDialog => todo!(),
        }
    }
}

#[derive(Debug)]
pub struct RegistryDataEntry {
    pub entry_id: Identifier,
    pub data: Option<nbt::NbtTag>,
}

impl ProtocolWrite for RegistryDataEntry {
    fn write_all<W: io::Write>(&self, writer: &mut W) -> KeisteenResult<()> {
        self.entry_id.write_all(writer)?;
        self.data.prefixed_write_all(writer)?;
        Ok(())
    }
}

#[derive(Debug)]
pub enum SConfigPacket {
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
    AcknowledgeFinishConfig,
    KeepAlive,
    Pong,
    ResourcePackResponse,
    KnownPacks {
        known_packs: Vec<KnownPack>,
    },
    CustomClickAction,
}

impl ServerboundPacket for SConfigPacket {
    fn decode(mut raw: RawPacket) -> KeisteenResult<Self> {
        match raw.packet_id.raw() {
            0x00 => Ok(Self::ClientInformation {
                locale: raw.data.read()?,
                view_distance: raw.data.read()?,
                chat_mode: raw.data.read()?,
                chat_colors: raw.data.read()?,
                displayed_skin_parts: raw.data.read()?,
                main_hand: raw.data.read()?,
                enable_text_filtering: raw.data.read()?,
                allow_server_listing: raw.data.read()?,
                particle_status: raw.data.read()?,
            }),
            0x01 => todo!(),
            0x02 => {
                let channel = raw.data.read()?;
                let data_len = raw.data.bytes().len();
                Ok(Self::PluginMessage { channel, data: raw.data.read_predefined(data_len)? })
            }
            0x03 => Ok(Self::AcknowledgeFinishConfig),
            0x04 => todo!(),
            0x05 => todo!(),
            0x06 => todo!(),
            0x07 => Ok(Self::KnownPacks { known_packs: raw.data.read_prefixed()? }),
            0x08 => todo!(),
            id => Self::handle_invalid_packet_id(id),
        }
    }
}

#[derive(Debug)]
pub struct KnownPack {
    pub namespace: String,
    pub id: String,
    pub version: String,
}

impl ProtocolRead for KnownPack {
    fn read_from<R: io::Read>(reader: &mut R) -> KeisteenResult<Self> {
        Ok(Self {
            namespace: String::read_from(reader)?,
            id: String::read_from(reader)?,
            version: String::read_from(reader)?,
        })
    }
}

impl ProtocolWrite for KnownPack {
    fn write_all<W: io::Write>(&self, writer: &mut W) -> KeisteenResult<()> {
        self.namespace.write_all(writer)?;
        self.id.write_all(writer)?;
        self.version.write_all(writer)?;
        Ok(())
    }
}
