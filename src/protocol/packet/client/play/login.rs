use crate::protocol::packet::PacketData;
use crate::protocol::packet::client::ClientboundPacket;
use crate::types::{Identifier, Position, VarInt};

#[derive(Debug)]
pub struct Login {
    entity_id: i32,
    is_hardcore: bool,
    dimension_names: Vec<Identifier>,
    max_players: VarInt,
    view_distance: VarInt,
    simulation_distance: VarInt,
    reduced_debug_info: bool,
    enable_respawn_screen: bool,
    do_limited_crafting: bool,
    dimension_type: VarInt,
    dimension_name: Identifier,
    hashed_seed: i64,
    game_mode: u8,
    previous_game_mode: i8,
    is_debug: bool,
    is_flat: bool,
    has_death_location: bool,
    death_dimension: Option<Identifier>,
    death_location: Option<Position>,
    portal_cooldown: VarInt,
    sea_level: VarInt,
    enforces_secure_chat: bool,
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
