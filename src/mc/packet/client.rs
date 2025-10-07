use std::fmt::Debug;

use bytes::BytesMut;

pub mod config;
pub mod login;
pub mod status;

pub trait ClientboundPacket: Debug {
    const PACKET_ID: i32;

    fn encode_data(self, bytes: &mut BytesMut)
    where
        Self: Sized;
}
