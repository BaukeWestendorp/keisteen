use crate::protocol::packet::PacketData;
use crate::protocol::packet::client::ClientboundPacket;
use crate::types::{Identifier, Position, VarInt};

#[derive(Debug)]
pub struct Login {
    pub entity_id: i32,
    pub is_hardcore: bool,
    pub dimension_names: Vec<Identifier>,
    pub max_players: VarInt,
    pub view_distance: VarInt,
    pub simulation_distance: VarInt,
    pub reduced_debug_info: bool,
    pub enable_respawn_screen: bool,
    pub do_limited_crafting: bool,
    pub dimension_type: VarInt,
    pub dimension_name: Identifier,
    pub hashed_seed: i64,
    pub game_mode: u8,
    pub previous_game_mode: i8,
    pub is_debug: bool,
    pub is_flat: bool,
    pub has_death_location: bool,
    pub death_dimension: Option<Identifier>,
    pub death_location: Option<Position>,
    pub portal_cooldown: VarInt,
    pub sea_level: VarInt,
    pub enforces_secure_chat: bool,
}

impl ClientboundPacket for Login {
    const PACKET_ID: i32 = 0x2B;

    fn encode(self, data: &mut PacketData) {
        data.write(self.entity_id);
        data.write(self.is_hardcore);
        data.write_prefixed(self.dimension_names);
        data.write(self.max_players);
        data.write(self.view_distance);
        data.write(self.simulation_distance);
        data.write(self.reduced_debug_info);
        data.write(self.enable_respawn_screen);
        data.write(self.do_limited_crafting);
        data.write(self.dimension_type);
        data.write(self.dimension_name);
        data.write(self.hashed_seed);
        data.write(self.game_mode);
        data.write(self.previous_game_mode);
        data.write(self.is_debug);
        data.write(self.is_flat);
        data.write(self.has_death_location);
        if self.has_death_location {
            data.write(self.death_dimension);
        }
        if self.has_death_location {
            data.write(self.death_location);
        }
        data.write(self.portal_cooldown);
        data.write(self.sea_level);
        data.write(self.enforces_secure_chat);
    }
}
