use bytes::{Buf, BufMut, BytesMut};
use tokio::io;
use tokio_util::codec::{Decoder, Encoder};

use crate::mc::network::protocol::{BytesExt, BytesMutExt};
use crate::mc::network::varint::VarInt;
use crate::mc::packet::{ClientboundRawPacket, ServerboundRawPacket};

const MAX_PACKET_SIZE: usize = 2097151; // 2^21 - 1

pub struct PacketCodec;

impl Encoder<ClientboundRawPacket> for PacketCodec {
    type Error = io::Error;

    fn encode(
        &mut self,
        packet: ClientboundRawPacket,
        destination: &mut BytesMut,
    ) -> Result<(), Self::Error> {
        destination.put_varint(VarInt::new(packet.length()));
        destination.put_varint(packet.id);
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

        let mut bytes = src.clone().freeze();

        let packet_length = match bytes.try_get_varint() {
            Ok(packet_length) => packet_length,
            Err(_) => return Ok(None), // Not enough data to read length
        };

        src.advance(packet_length.byte_count());
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

        let id = match bytes.try_get_varint() {
            Ok(id) => id,
            Err(_) => return Ok(None), // Not enough data to read packet id
        };

        src.advance(id.byte_count());

        let data = src.clone().freeze();

        src.advance(data.len());

        Ok(Some(ServerboundRawPacket { id, data }))
    }
}
