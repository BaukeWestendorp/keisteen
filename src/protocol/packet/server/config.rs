use std::collections::BTreeMap;

use eyre::bail;

use crate::error::KeisteenResult;
use crate::nbt;
use crate::protocol::packet::known_pack::KnownPack;
use crate::protocol::packet::registry_data_entry::RegistryDataEntry;
use crate::protocol::packet::server::ServerboundPacket;
use crate::protocol::packet::{RawPacket, client};
use crate::protocol::registry::Registry;
use crate::server::conn::{Connection, ConnectionState};
use crate::types::{Identifier, VarInt};

pub fn handle_raw_packet(raw: RawPacket, conn: &mut Connection) -> KeisteenResult<()> {
    match raw.packet_id.raw() {
        AcknowledgeFinishConfig::PACKET_ID => AcknowledgeFinishConfig::decode(raw)?.handle(conn),
        ClientInformation::PACKET_ID => ClientInformation::decode(raw)?.handle(conn),
        CookieResponse::PACKET_ID => CookieResponse::decode(raw)?.handle(conn),
        CustomClickAction::PACKET_ID => CustomClickAction::decode(raw)?.handle(conn),
        KeepAlive::PACKET_ID => KeepAlive::decode(raw)?.handle(conn),
        KnownPacks::PACKET_ID => KnownPacks::decode(raw)?.handle(conn),
        PluginMessage::PACKET_ID => PluginMessage::decode(raw)?.handle(conn),
        Pong::PACKET_ID => Pong::decode(raw)?.handle(conn),
        ResourcePackResponse::PACKET_ID => ResourcePackResponse::decode(raw)?.handle(conn),
        _ => bail!("unknown config packet id: {}", raw.packet_id.raw()),
    }
}

#[derive(Debug)]
pub struct ClientInformation {
    pub locale: String,
    pub view_distance: i8,
    pub chat_mode: VarInt,
    pub chat_colors: bool,
    pub displayed_skin_parts: u8,
    pub main_hand: VarInt,
    pub enable_text_filtering: bool,
    pub allow_server_listing: bool,
    pub particle_status: VarInt,
}

impl ServerboundPacket for ClientInformation {
    const PACKET_ID: i32 = 0x00;

    fn decode(mut raw: RawPacket) -> KeisteenResult<Self> {
        Ok(Self {
            locale: raw.data.read()?,
            view_distance: raw.data.read()?,
            chat_mode: raw.data.read()?,
            chat_colors: raw.data.read()?,
            displayed_skin_parts: raw.data.read()?,
            main_hand: raw.data.read()?,
            enable_text_filtering: raw.data.read()?,
            allow_server_listing: raw.data.read()?,
            particle_status: raw.data.read()?,
        })
    }

    fn handle(&self, _conn: &mut Connection) -> KeisteenResult<()> {
        // TODO: Do something with client information.
        Ok(())
    }
}

#[derive(Debug)]
pub struct CookieResponse;

impl ServerboundPacket for CookieResponse {
    const PACKET_ID: i32 = 0x01;

    fn decode(_raw: RawPacket) -> KeisteenResult<Self> {
        todo!()
    }

    fn handle(&self, _conn: &mut Connection) -> KeisteenResult<()> {
        todo!()
    }
}

#[derive(Debug)]
pub struct PluginMessage {
    pub channel: Identifier,
    pub data: Vec<u8>,
}

impl ServerboundPacket for PluginMessage {
    const PACKET_ID: i32 = 0x02;

    fn decode(mut raw: RawPacket) -> KeisteenResult<Self> {
        let channel = raw.data.read()?;
        let data_len = raw.data.bytes().len();
        Ok(Self { channel, data: raw.data.read_predefined(data_len)? })
    }

    fn handle(&self, _conn: &mut Connection) -> KeisteenResult<()> {
        if self.channel.namespace() == "minecraft" && self.channel.value() == "brand" {
            let brand_string = str::from_utf8(&self.data)?;
            log::debug!("client brand: \"{}\"", brand_string);
        } else {
            log::debug!("received channel message on channel '{}': {:?}", self.channel, self.data);
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct AcknowledgeFinishConfig;

impl ServerboundPacket for AcknowledgeFinishConfig {
    const PACKET_ID: i32 = 0x03;

    fn decode(_raw: RawPacket) -> KeisteenResult<Self> {
        Ok(Self)
    }

    fn handle(&self, conn: &mut Connection) -> KeisteenResult<()> {
        log::debug!("configuration acknowledged");
        conn.state = ConnectionState::Play;
        Ok(())
    }
}

#[derive(Debug)]
pub struct KeepAlive;

impl ServerboundPacket for KeepAlive {
    const PACKET_ID: i32 = 0x04;

    fn decode(_raw: RawPacket) -> KeisteenResult<Self> {
        todo!()
    }

    fn handle(&self, _conn: &mut Connection) -> KeisteenResult<()> {
        todo!()
    }
}

#[derive(Debug)]
pub struct Pong;

impl ServerboundPacket for Pong {
    const PACKET_ID: i32 = 0x05;

    fn decode(_raw: RawPacket) -> KeisteenResult<Self> {
        todo!()
    }

    fn handle(&self, _conn: &mut Connection) -> KeisteenResult<()> {
        todo!()
    }
}

#[derive(Debug)]
pub struct ResourcePackResponse;

impl ServerboundPacket for ResourcePackResponse {
    const PACKET_ID: i32 = 0x06;

    fn decode(_raw: RawPacket) -> KeisteenResult<Self> {
        todo!()
    }

    fn handle(&self, _conn: &mut Connection) -> KeisteenResult<()> {
        todo!()
    }
}

#[derive(Debug)]
pub struct KnownPacks {
    pub known_packs: Vec<KnownPack>,
}

impl ServerboundPacket for KnownPacks {
    const PACKET_ID: i32 = 0x07;

    fn decode(mut raw: RawPacket) -> KeisteenResult<Self> {
        Ok(Self { known_packs: raw.data.read_prefixed()? })
    }

    fn handle(&self, conn: &mut Connection) -> KeisteenResult<()> {
        log::debug!("client's known packs: {:?}", self.known_packs);

        // TODO: Actually synchronize known packs.

        self.send_registry_data_packets(conn)?;

        // TODO: Update Tags

        conn.send_packet(client::config::FinishConfig)?;
        log::debug!("configuration finished");

        Ok(())
    }
}

impl KnownPacks {
    fn send_registry_data_packets(&self, conn: &mut Connection) -> KeisteenResult<()> {
        let packets = {
            let server = conn.server().read();
            let registries = server.registries();
            vec![
                create_packet(registries.banner_pattern())?,
                create_packet(registries.cat_variant())?,
                create_packet(registries.chat_type())?,
                create_packet(registries.chicken_variant())?,
                create_packet(registries.cow_variant())?,
                create_packet(registries.damage_type())?,
                // TODO: create_packet(registries.dialog())?,
                create_packet(registries.dimension_type())?,
                create_packet(registries.frog_variant())?,
                create_packet(registries.painting_variant())?,
                create_packet(registries.pig_variant())?,
                // TODO: create_packet(registries.trim_material())?,
                // TODO: create_packet(registries.trim_pattern())?,
                create_packet(registries.wolf_sound_variant())?,
                create_packet(registries.wolf_variant())?,
                // TODO: create_packet(registries.worldgen_biome())?,
            ]
        };

        for packet in packets {
            conn.send_packet(packet)?;
        }

        fn create_packet<R: Registry + serde::Serialize>(
            registry_entries: &BTreeMap<Identifier, R>,
        ) -> KeisteenResult<client::config::RegistryData> {
            Ok(client::config::RegistryData {
                registry_id: R::identifier(),
                entries: {
                    let mut entries = Vec::new();
                    for (identifier, entry) in registry_entries {
                        let entry_nbt = nbt::to_value(entry)?;
                        entries.push(RegistryDataEntry {
                            entry_id: identifier.clone(),
                            data: Some(entry_nbt),
                        });
                    }
                    entries
                },
            })
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct CustomClickAction;

impl ServerboundPacket for CustomClickAction {
    const PACKET_ID: i32 = 0x08;

    fn decode(_raw: RawPacket) -> KeisteenResult<Self> {
        todo!()
    }

    fn handle(&self, _conn: &mut Connection) -> KeisteenResult<()> {
        todo!()
    }
}
