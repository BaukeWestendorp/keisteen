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

impl From<CPlayPacket> for RawPacket {
    fn from(packet: CPlayPacket) -> Self {
        match packet {
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
            } => RawPacket {
                packet_id: VarInt::new(0x2B),
                data: {
                    let mut data = PacketData::new();
                    data.write_all(entity_id);
                    data.write_all(is_hardcore);
                    data.write_all_prefixed(dimension_names);
                    data.write_all(max_players);
                    data.write_all(view_distance);
                    data.write_all(simulation_distance);
                    data.write_all(reduced_debug_info);
                    data.write_all(enable_respawn_screen);
                    data.write_all(do_limited_crafting);
                    data.write_all(dimension_type);
                    data.write_all(dimension_name);
                    data.write_all(hashed_seed);
                    data.write_all(game_mode);
                    data.write_all(previous_game_mode);
                    data.write_all(is_debug);
                    data.write_all(is_flat);
                    data.write_all(has_death_location);
                    if has_death_location {
                        data.write_all(death_dimension);
                    }
                    if has_death_location {
                        data.write_all(death_location);
                    }
                    data.write_all(portal_cooldown);
                    data.write_all(sea_level);
                    data.write_all(enforces_secure_chat);
                    data
                },
            },
        }
    }
}

#[derive(Debug)]
pub enum SPlayPacket {
    StatusRequest,
    PingRequest { timestamp: i64 },
}

impl TryFrom<RawPacket> for SPlayPacket {
    type Error = crate::error::Error;

    fn try_from(packet: RawPacket) -> Result<Self, Self::Error> {
        match packet.packet_id.raw() {
            packet_id => todo!("implement play packet {packet_id}"),
        }
    }
}
