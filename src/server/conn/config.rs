use crate::error::CraftError;
use crate::protocol::packet::{CConfigurationPacket, CPlayPacket, SConfigurationPacket};
use crate::server::conn::{Connection, ConnectionState};
use crate::types::{Identifier, VarInt};

impl Connection {
    pub fn handle_configuration_packet(
        &mut self,
        packet: SConfigurationPacket,
    ) -> Result<(), CraftError> {
        match packet {
            SConfigurationPacket::ClientInformation { .. } => {
                // TODO: Do something with client information.
                self.write_raw_packet(CConfigurationPacket::FinishConfiguration)?;
            }
            SConfigurationPacket::CookieResponse => todo!(),
            SConfigurationPacket::PluginMessage { channel, data } => {
                tracing::debug!("received channel message on channel '{channel}': {data:?}");
            }
            SConfigurationPacket::AcknowledgeFinishConfiguration => {
                tracing::debug!("configuration acknowledged");
                self.state = ConnectionState::Play;

                self.write_raw_packet(CPlayPacket::Login {
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
