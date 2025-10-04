use std::collections::BTreeMap;

use crate::error::KeisteenResult;
use crate::nbt;
use crate::protocol::packet::known_pack::KnownPack;
use crate::protocol::packet::registry_data_entry::RegistryDataEntry;
use crate::protocol::packet::server::ServerboundPacket;
use crate::protocol::packet::{RawPacket, client};
use crate::protocol::registry::Registry;
use crate::server::conn::Connection;
use crate::types::Identifier;

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
