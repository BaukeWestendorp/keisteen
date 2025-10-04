use crate::error::KeisteenResult;
use crate::protocol::packet::{ClientboundPacket, ServerboundPacket};

use super::{PacketData, RawPacket};

#[derive(Debug)]
pub enum CStatusPacket {
    StatusResponse { json_response: String },
    PongResponse { timestamp: i64 },
}

impl ClientboundPacket for CStatusPacket {
    fn encode(self, data: &mut PacketData) {
        match self {
            CStatusPacket::StatusResponse { json_response } => {
                data.write_all(json_response);
            }
            CStatusPacket::PongResponse { timestamp } => {
                data.write_all(timestamp);
            }
        }
    }

    fn packet_id(&self) -> i32 {
        match self {
            CStatusPacket::StatusResponse { .. } => 0x00,
            CStatusPacket::PongResponse { .. } => 0x01,
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
