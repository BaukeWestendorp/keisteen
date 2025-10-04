use std::collections::BTreeMap;

use crate::error::KeisteenResult;
use crate::nbt;
use crate::protocol::packet::{
    CConfigurationPacket, KnownPack, ProtocolWrite, RegistryDataEntry, SConfigurationPacket,
};
use crate::protocol::registry::Registry;
use crate::server::conn::{Connection, ConnectionState};
use crate::types::Identifier;

impl Connection {
    pub fn handle_configuration_packet(
        &mut self,
        packet: SConfigurationPacket,
    ) -> KeisteenResult<()> {
        match packet {
            SConfigurationPacket::ClientInformation { .. } => {
                // TODO: Do something with client information.
            }
            SConfigurationPacket::CookieResponse => todo!(),
            SConfigurationPacket::PluginMessage { channel, data } => {
                if channel.namespace() == "minecraft" && channel.value() == "brand" {
                    let brand_string = str::from_utf8(&data)?;
                    log::debug!("client brand: \"{}\"", brand_string);
                } else {
                    log::debug!("received channel message on channel '{channel}': {data:?}");
                }
            }
            SConfigurationPacket::AcknowledgeFinishConfiguration => {
                log::debug!("configuration acknowledged");
                self.state = ConnectionState::Play;
            }
            SConfigurationPacket::KeepAlive => todo!(),
            SConfigurationPacket::Pong => todo!(),
            SConfigurationPacket::ResourcePackResponse => todo!(),
            SConfigurationPacket::KnownPacks { known_packs } => {
                log::debug!("client's known packs: {known_packs:?}");
                // TODO: Do something with known packs.

                self.send_registry_data_packets()?;

                // TODO: Update Tags

                self.finish_configuration()?;
            }
            SConfigurationPacket::CustomClickAction => todo!(),
        }

        Ok(())
    }

    pub fn start_configuration(&mut self) -> KeisteenResult<()> {
        self.state = ConnectionState::Configuration;
        self.send_brand_plugin_message_packet(crate::BRAND)?;
        // TODO: Send Feature Flags
        self.send_known_packs_packet()?;
        Ok(())
    }

    fn send_brand_plugin_message_packet(&mut self, brand: &str) -> KeisteenResult<()> {
        let mut data = Vec::new();
        ProtocolWrite::write_all(brand, &mut data)?;

        self.send_packet(CConfigurationPacket::PluginMessage {
            channel: Identifier::new("minecraft", "brand")?,
            data,
        })?;

        Ok(())
    }

    fn send_known_packs_packet(&mut self) -> KeisteenResult<()> {
        // TODO: Actually synchronize known packs.
        let known_packs = vec![KnownPack {
            namespace: "minecraft".to_string(),
            id: "core".to_string(),
            version: crate::MC_VERSION.to_string(),
        }];

        self.send_packet(CConfigurationPacket::KnownPacks { known_packs })?;

        Ok(())
    }

    fn send_registry_data_packets(&mut self) -> KeisteenResult<()> {
        let packets = {
            let server = self.server.read();
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
            self.send_packet(packet)?;
        }

        fn create_packet<R: Registry + serde::Serialize>(
            registry_entries: &BTreeMap<Identifier, R>,
        ) -> KeisteenResult<CConfigurationPacket> {
            Ok(CConfigurationPacket::RegistryData {
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

    fn finish_configuration(&mut self) -> KeisteenResult<()> {
        self.send_packet(CConfigurationPacket::FinishConfiguration)?;

        log::debug!("configuration finished");

        Ok(())
    }
}
