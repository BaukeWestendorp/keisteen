use std::io::{self};

use aes::cipher::generic_array::GenericArray;
use aes::cipher::{BlockDecryptMut, BlockEncryptMut, BlockSizeUser, KeyIvInit};

pub enum EncryptionStream<W: io::Write> {
    Unencrypted(Option<W>),
    Encrypted { cipher: cfb8::Encryptor<aes::Aes128>, writer: W },
}

impl<W: io::Write> EncryptionStream<W> {
    pub fn new(writer: W) -> Self {
        Self::Unencrypted(Some(writer))
    }

    pub fn writer(&self) -> &W {
        match self {
            Self::Unencrypted(writer) => writer.as_ref().unwrap(),
            Self::Encrypted { writer, .. } => writer,
        }
    }

    fn writer_mut(&mut self) -> &mut W {
        match self {
            Self::Unencrypted(writer) => writer.as_mut().unwrap(),
            Self::Encrypted { writer, .. } => writer,
        }
    }

    pub fn enable_encryption(&mut self, shared_secret: &[u8]) {
        match self {
            EncryptionStream::Unencrypted(writer) => {
                let cipher =
                    cfb8::Encryptor::new_from_slices(shared_secret, shared_secret).unwrap();
                *self = Self::Encrypted { cipher, writer: writer.take().unwrap() }
            }
            EncryptionStream::Encrypted { .. } => {}
        }
    }
}

impl<W: io::Write> io::Write for EncryptionStream<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            Self::Unencrypted(writer) => writer.as_mut().unwrap().write(buf),
            Self::Encrypted { cipher, writer } => {
                let block_size = cfb8::Encryptor::<aes::Aes128>::block_size();
                let mut written_bytes = 0;
                for block in buf.chunks(block_size) {
                    let mut out = [0u8];

                    let out_block = GenericArray::from_mut_slice(&mut out);
                    cipher.encrypt_block_b2b_mut(block.into(), out_block);

                    let written = writer.write(&out)?;
                    written_bytes += written;
                }
                Ok(written_bytes)
            }
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer_mut().flush()
    }
}

pub enum DecryptionStream<R: io::Read> {
    Unencrypted(Option<R>),
    Encrypted { cipher: cfb8::Decryptor<aes::Aes128>, reader: R },
}

impl<R: io::Read> DecryptionStream<R> {
    pub fn new(reader: R) -> Self {
        Self::Unencrypted(Some(reader))
    }

    pub fn enable_encryption(&mut self, shared_secret: &[u8]) {
        match self {
            DecryptionStream::Unencrypted(reader) => {
                let cipher =
                    cfb8::Decryptor::new_from_slices(shared_secret, shared_secret).unwrap();
                *self = Self::Encrypted { cipher, reader: reader.take().unwrap() }
            }
            DecryptionStream::Encrypted { .. } => {}
        }
    }
}

impl<R: io::Read> io::Read for DecryptionStream<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            Self::Unencrypted(reader) => reader.as_mut().unwrap().read(buf),
            Self::Encrypted { cipher, reader } => {
                let block_size = cfb8::Decryptor::<aes::Aes128>::block_size();
                let mut bytes_read = 0;
                for block in buf.chunks_mut(block_size) {
                    let read = reader.read(block)?;
                    cipher.decrypt_block_mut(block.into());
                    bytes_read += read;
                }
                Ok(bytes_read)
            }
        }
    }
}
