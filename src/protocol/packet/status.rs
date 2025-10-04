use crate::error::KeisteenResult;
use crate::protocol::packet::ServerboundPacket;
use crate::types::VarInt;

use super::{PacketData, RawPacket};

#[derive(Debug)]
pub enum CStatusPacket {
    StatusResponse { json_response: String },
    PongResponse { timestamp: i64 },
}

impl From<CStatusPacket> for RawPacket {
    fn from(packet: CStatusPacket) -> Self {
        match packet {
            CStatusPacket::StatusResponse { json_response } => RawPacket {
                packet_id: VarInt::new(0x00),
                data: {
                    let mut data = PacketData::new();
                    data.write_all(json_response);
                    data
                },
            },
            CStatusPacket::PongResponse { timestamp } => RawPacket {
                packet_id: VarInt::new(0x01),
                data: {
                    let mut data = PacketData::new();
                    data.write_all(timestamp);
                    data
                },
            },
        }
    }
}

#[derive(Debug)]
pub enum SStatusPacket {
    StatusRequest,
    PingRequest { timestamp: i64 },
}

impl ServerboundPacket for SStatusPacket {
    fn decode(mut raw: RawPacket) -> KeisteenResult<Self> {
        match raw.packet_id.raw() {
            0x00 => Ok(Self::StatusRequest),
            0x01 => Ok(Self::PingRequest { timestamp: raw.data.read()? }),
            id => Self::handle_invalid_packet_id(id),
        }
    }
}
