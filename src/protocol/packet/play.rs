use crate::error::KeisteenResult;
use crate::protocol::packet::{ClientboundPacket, ServerboundPacket};
use crate::types::{Identifier, Position, VarInt};

use super::{PacketData, RawPacket};

#[derive(Debug)]
pub enum CPlayPacket {
    Login {
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
    },
}

impl ClientboundPacket for CPlayPacket {
    fn encode(self, data: &mut PacketData) {
        match self {
            CPlayPacket::Login {
                entity_id,
                is_hardcore,
                dimension_names,
                max_players,
                view_distance,
                simulation_distance,
                reduced_debug_info,
                enable_respawn_screen,
                do_limited_crafting,
                dimension_type,
                dimension_name,
                hashed_seed,
                game_mode,
                previous_game_mode,
                is_debug,
                is_flat,
                has_death_location,
                death_dimension,
                death_location,
                portal_cooldown,
                sea_level,
                enforces_secure_chat,
            } => {
                data.write(entity_id);
                data.write(is_hardcore);
                data.write_prefixed(dimension_names);
                data.write(max_players);
                data.write(view_distance);
                data.write(simulation_distance);
                data.write(reduced_debug_info);
                data.write(enable_respawn_screen);
                data.write(do_limited_crafting);
                data.write(dimension_type);
                data.write(dimension_name);
                data.write(hashed_seed);
                data.write(game_mode);
                data.write(previous_game_mode);
                data.write(is_debug);
                data.write(is_flat);
                data.write(has_death_location);
                if has_death_location {
                    data.write(death_dimension);
                }
                if has_death_location {
                    data.write(death_location);
                }
                data.write(portal_cooldown);
                data.write(sea_level);
                data.write(enforces_secure_chat);
            }
        }
    }

    fn packet_id(&self) -> i32 {
        match self {
            CPlayPacket::Login { .. } => 0x2B,
        }
    }
}

#[derive(Debug)]
pub enum SPlayPacket {
    StatusRequest,
    PingRequest { timestamp: i64 },
}

impl ServerboundPacket for SPlayPacket {
    fn decode(raw: RawPacket) -> KeisteenResult<Self> {
        match raw.packet_id.raw() {
            id => todo!("implement play packet {id}"),
        }
    }
}
