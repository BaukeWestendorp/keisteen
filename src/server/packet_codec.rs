use bytes::{Buf, BufMut, BytesMut};
use tokio::io;
use tokio_util::codec::{Decoder, Encoder};

use crate::mc::packet::{ClientboundRawPacket, ServerboundRawPacket};
use crate::mc::protocol::ProtocolWrite;
use crate::mc::types::VarInt;

const MAX_PACKET_SIZE: usize = 2097151; // 2^21 - 1

pub struct PacketCodec;

impl Encoder<ClientboundRawPacket> for PacketCodec {
    type Error = io::Error;

    fn encode(
        &mut self,
        packet: ClientboundRawPacket,
        destination: &mut BytesMut,
    ) -> Result<(), Self::Error> {
        VarInt::new(packet.length()).write(destination);
        packet.id.write(destination);
        destination.put(packet.data.freeze());
        Ok(())
    }
}

impl Decoder for PacketCodec {
    type Item = ServerboundRawPacket;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.is_empty() {
            return Ok(None);
        }

        let mut reader = src.clone().reader();

        let packet_length = match VarInt::from_reader(&mut reader) {
            Ok(packet_length) => packet_length,
            Err(_) => return Ok(None), // Not enough data to read length
        };

        src.advance(packet_length.len());
        let length = packet_length.raw() as usize;

        if length > MAX_PACKET_SIZE {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("packet length {} exceeds maximum {}", length, MAX_PACKET_SIZE),
            ));
        }

        if src.len() < length {
            // The full packet has not arrived yet.
            // We reserve more space in the buffer. This is not strictly
            // necessary, but is a good idea performance-wise.
            src.reserve(length - src.len());

            return Ok(None);
        }

        let id = match VarInt::from_reader(&mut reader) {
            Ok(id) => id,
            Err(_) => return Ok(None), // Not enough data to read packet id
        };

        src.advance(id.len() as usize);

        let data = src.clone().freeze();
        Ok(Some(ServerboundRawPacket { id, data }))
    }
}
