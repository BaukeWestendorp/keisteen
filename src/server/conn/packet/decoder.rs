use std::io;

use aes::cipher::KeyIvInit;
use eyre::bail;

use crate::error::KeisteenResult;
use crate::mc::protocol::packet::{PacketData, RawPacket};
use crate::mc::types::VarInt;
use crate::server::crypt::DecryptionStream;

pub enum PacketDecoder<R: io::Read> {
    Raw(Option<R>),
    Encrypted(Option<DecryptionStream<R>>),
    Compressed { reader: DecryptionStream<R> },
}

impl<R: io::Read> PacketDecoder<R> {
    pub fn new(reader: R) -> Self {
        Self::Raw(Some(reader))
    }

    pub fn enable_encryption(&mut self, shared_secret: &[u8]) -> KeisteenResult<()> {
        let reader = match self {
            Self::Raw(reader) => reader.take().unwrap(),
            Self::Encrypted(_) | Self::Compressed { .. } => bail!("encryption already enabled"),
        };

        let cipher = cfb8::Decryptor::new_from_slices(shared_secret, shared_secret).unwrap();
        let decryption_stream = DecryptionStream::new(cipher, reader);
        *self = Self::Encrypted(Some(decryption_stream));

        Ok(())
    }

    pub fn enable_compression(&mut self) -> KeisteenResult<()> {
        let reader = match self {
            Self::Raw(_) => bail!("stream is not encrypted"),
            Self::Encrypted(reader) => reader.take().unwrap(),
            Self::Compressed { .. } => bail!("compression already enabled"),
        };

        *self = Self::Compressed { reader };

        Ok(())
    }

    pub fn read_packet(&mut self) -> io::Result<RawPacket> {
        fn read_uncompresed<R: io::Read>(mut reader: R, length: VarInt) -> io::Result<RawPacket> {
            let packet_id = VarInt::from_reader(&mut reader)?;
            let data_len = (length.raw() as usize).saturating_sub(packet_id.len());
            let mut data = vec![0u8; data_len];
            reader.read_exact(&mut data)?;
            Ok(RawPacket { packet_id, data: PacketData::from(data) })
        }

        let packet = match self {
            PacketDecoder::Raw(Some(reader)) => {
                let length = VarInt::from_reader(reader)?;
                read_uncompresed(reader, length)?
            }
            PacketDecoder::Encrypted(Some(reader)) => {
                let length = VarInt::from_reader(reader)?;
                read_uncompresed(reader, length)?
            }
            PacketDecoder::Compressed { reader } => {
                // TODO: Implement compression.

                let length = VarInt::from_reader(reader)?;
                read_uncompresed(reader, length)?
            }
            _ => unreachable!(),
        };

        Ok(packet)
    }
}
