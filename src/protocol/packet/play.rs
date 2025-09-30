use crate::error::CraftError;
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
                    data.write_i32(entity_id);
                    data.write_bool(is_hardcore);
                    data.write_prefixed_identifier_array(dimension_names);
                    data.write_varint(max_players);
                    data.write_varint(view_distance);
                    data.write_varint(simulation_distance);
                    data.write_bool(reduced_debug_info);
                    data.write_bool(enable_respawn_screen);
                    data.write_bool(do_limited_crafting);
                    data.write_varint(dimension_type);
                    data.write_identifier(&dimension_name);
                    data.write_i64(hashed_seed);
                    data.write_u8(game_mode);
                    data.write_i8(previous_game_mode);
                    data.write_bool(is_debug);
                    data.write_bool(is_flat);
                    data.write_bool(has_death_location);
                    if has_death_location {
                        data.write_identifier(&death_dimension.unwrap());
                    }
                    if has_death_location {
                        data.write_position(death_location.unwrap());
                    }
                    data.write_varint(portal_cooldown);
                    data.write_varint(sea_level);
                    data.write_bool(enforces_secure_chat);
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
    type Error = CraftError;

    fn try_from(packet: RawPacket) -> Result<Self, Self::Error> {
        match packet.packet_id.raw() {
            packet_id => todo!("implement play packet {packet_id}"),
        }
    }
}
