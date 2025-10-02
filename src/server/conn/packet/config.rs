use crate::protocol::packet::SConfigurationPacket;
use crate::server::conn::{Connection, ConnectionState};

impl Connection {
    pub fn handle_configuration_packet(
        &mut self,
        packet: SConfigurationPacket,
    ) -> crate::error::Result<()> {
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

                self.send_packet(CPlayPacket::Login {
                    entity_id: 1,
                    is_hardcore: false,
                    dimension_names: vec![Identifier::new(
                        "minecraft".to_string(),
                        "overworld".to_string(),
                    )?],
                    max_players: VarInt::new(20),
                    view_distance: VarInt::new(10),
                    simulation_distance: VarInt::new(10),
                    reduced_debug_info: false,
                    enable_respawn_screen: true,
                    do_limited_crafting: false,
                    dimension_type: VarInt::new(1),
                    dimension_name: Identifier::new(
                        "minecraft".to_string(),
                        "overworld".to_string(),
                    )?,
                    hashed_seed: 0,
                    game_mode: 1,
                    previous_game_mode: -1,
                    is_debug: false,
                    is_flat: false,
                    has_death_location: false,
                    death_dimension: None,
                    death_location: None,
                    portal_cooldown: VarInt::new(0),
                    sea_level: VarInt::new(63),
                    enforces_secure_chat: false,
                })?;
            }
            SConfigurationPacket::KeepAlive => todo!(),
            SConfigurationPacket::Pong => todo!(),
            SConfigurationPacket::ResourcePackResponse => todo!(),
            SConfigurationPacket::KnownPacks => todo!(),
            SConfigurationPacket::CustomClickAction => todo!(),
        }

        Ok(())
    }
}
