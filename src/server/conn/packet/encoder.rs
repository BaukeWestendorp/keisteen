use std::io;

use aes::cipher::KeyIvInit;
use eyre::bail;

use crate::error::KeisteenResult;
use crate::mc::protocol::packet::PacketData;
use crate::mc::protocol::packet::client::ClientboundPacket;
use crate::mc::types::VarInt;
use crate::server::crypt::EncryptionStream;

pub enum PacketEncoder<W: io::Write> {
    Raw(Option<W>),
    Encrypted(Option<EncryptionStream<W>>),
    Compressed { writer: EncryptionStream<W>, threshold: u32, level: u32 },
}

impl<W: io::Write> PacketEncoder<W> {
    pub fn new(writer: W) -> Self {
        Self::Raw(Some(writer))
    }

    pub fn enable_encryption(&mut self, shared_secret: &[u8]) -> KeisteenResult<()> {
        let writer = match self {
            Self::Raw(writer) => writer.take().unwrap(),
            Self::Encrypted(_) | Self::Compressed { .. } => bail!("encryption already enabled"),
        };

        let cipher = cfb8::Encryptor::new_from_slices(shared_secret, shared_secret).unwrap();
        let encryption_stream = EncryptionStream::new(cipher, writer);
        *self = Self::Encrypted(Some(encryption_stream));

        Ok(())
    }

    pub fn enable_compression(&mut self, threshold: u32, level: u32) -> KeisteenResult<()> {
        let writer = match self {
            Self::Raw(_) => bail!("stream is not encrypted"),
            Self::Encrypted(writer) => writer.take().unwrap(),
            Self::Compressed { .. } => bail!("compression already enabled"),
        };

        *self = Self::Compressed { writer, threshold, level };

        Ok(())
    }

    pub fn write_packet<P: ClientboundPacket>(&mut self, packet: P) -> io::Result<()> {
        let packet_id = VarInt::new(P::PACKET_ID);
        let mut data = PacketData::new();
        packet.encode(&mut data);
        let packet_length = VarInt::new((packet_id.len() + data.bytes().len()) as i32);

        let mut buf = Vec::new();
        packet_id.to_writer(&mut buf)?;
        data.to_writer(&mut buf)?;

        match self {
            PacketEncoder::Raw(Some(writer)) => {
                packet_length.to_writer(writer)?;
                packet_id.to_writer(writer)?;
                data.to_writer(writer)?;
            }
            PacketEncoder::Encrypted(Some(writer)) => {
                packet_length.to_writer(writer)?;
                packet_id.to_writer(writer)?;
                data.to_writer(writer)?;
            }
            PacketEncoder::Compressed { writer, threshold, level } => {
                // TODO: Implement compression.
                let _ = threshold;
                let _ = level;

                packet_length.to_writer(writer)?;
                packet_id.to_writer(writer)?;
                data.to_writer(writer)?;
            }
            _ => unreachable!(),
        }

        Ok(())
    }
}
