use std::fmt::Debug;

use crate::error::KeisteenResult;
use crate::server::connection::Connection;

use bytes::Bytes;

pub mod handshake;
pub mod login;
pub mod status;

pub trait ServerboundPacket: Debug {
    const PACKET_ID: i32;

    fn decode_data(bytes: Bytes) -> KeisteenResult<Self>
    where
        Self: Sized;

    async fn handle(self, connection: &mut Connection) -> KeisteenResult<()>
    where
        Self: Sized;
}
