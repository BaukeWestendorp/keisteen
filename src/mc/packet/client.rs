use bytes::BytesMut;

pub use login::LoginSuccess;

pub mod login;

pub trait ClientboundPacket {
    const PACKET_ID: i32;

    fn encode_data(self, bytes: &mut BytesMut)
    where
        Self: Sized;
}
