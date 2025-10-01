use crate::error::CraftError;
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

impl TryFrom<RawPacket> for SStatusPacket {
    type Error = CraftError;

    fn try_from(mut packet: RawPacket) -> Result<Self, Self::Error> {
        match packet.packet_id.raw() {
            0x00 => Ok(Self::StatusRequest),
            0x01 => Ok(Self::PingRequest { timestamp: packet.data.read()? }),
            _ => todo!(),
        }
    }
}
