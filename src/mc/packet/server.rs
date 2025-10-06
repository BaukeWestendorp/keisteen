use crate::error::KeisteenResult;

use bytes::Bytes;

pub use handshake::Handshake;

pub mod handshake;

pub trait ServerboundPacket {
    const PACKET_ID: i32;

    fn decode_data(bytes: Bytes) -> KeisteenResult<Self>
    where
        Self: Sized;

    async fn handle(self) -> KeisteenResult<()>
    where
        Self: Sized;
}
