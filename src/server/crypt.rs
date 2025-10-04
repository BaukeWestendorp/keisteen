use std::io;

use aes::cipher::generic_array::GenericArray;
use aes::cipher::{BlockDecryptMut, BlockEncryptMut, BlockSizeUser};
use rsa::traits::PublicKeyParts;
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey};

use crate::protocol::packet::CLoginPacket;

pub struct CryptKeys {
    public_key_der: Vec<u8>,
    private_key: rsa::RsaPrivateKey,

    verification_token: [u8; 4],
}

impl CryptKeys {
    pub fn new() -> Self {
        let private_key = Self::generate_private_key();
        Self {
            public_key_der: rsa_der::public_key_to_der(
                &private_key.n().to_bytes_be(),
                &private_key.e().to_bytes_be(),
            ),
            private_key,

            verification_token: rand::random(),
        }
    }

    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, rsa::Error> {
        self.private_key.decrypt(Pkcs1v15Encrypt::default(), &data)
    }

    pub fn verify_token(&self, token: &[u8]) -> Result<bool, rsa::Error> {
        let verify_token = self.decrypt(token)?;
        Ok(verify_token == self.verification_token)
    }

    fn generate_private_key() -> RsaPrivateKey {
        let mut rng = rand::thread_rng();
        rsa::RsaPrivateKey::new(&mut rng, 1024).expect("failed to generate a key")
    }

    pub fn generate_encryption_request_packet(&self, should_authenticate: bool) -> CLoginPacket {
        CLoginPacket::EncryptionRequest {
            server_id: "".to_string(),
            public_key: self.public_key_der.clone(),
            verify_token: self.verification_token.to_vec(),
            should_authenticate,
        }
    }
}

pub struct EncryptionStream<W: io::Write> {
    cipher: cfb8::Encryptor<aes::Aes128>,
    writer: W,
}

impl<W: io::Write> EncryptionStream<W> {
    pub fn new(cipher: cfb8::Encryptor<aes::Aes128>, writer: W) -> Self {
        Self { cipher, writer }
    }
}

impl<W: io::Write> io::Write for EncryptionStream<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let block_size = cfb8::Encryptor::<aes::Aes128>::block_size();
        let mut written_bytes = 0;
        for block in buf.chunks(block_size) {
            let mut out = [0u8];

            let out_block = GenericArray::from_mut_slice(&mut out);
            self.cipher.encrypt_block_b2b_mut(block.into(), out_block);

            let written = self.writer.write(&out)?;
            written_bytes += written;
        }
        Ok(written_bytes)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

pub struct DecryptionStream<R: io::Read> {
    cipher: cfb8::Decryptor<aes::Aes128>,
    reader: R,
}

impl<R: io::Read> DecryptionStream<R> {
    pub fn new(cipher: cfb8::Decryptor<aes::Aes128>, reader: R) -> Self {
        Self { cipher, reader }
    }
}

impl<R: io::Read> io::Read for DecryptionStream<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let block_size = cfb8::Decryptor::<aes::Aes128>::block_size();
        let mut bytes_read = 0;
        for block in buf.chunks_mut(block_size) {
            let read = self.reader.read(block)?;
            self.cipher.decrypt_block_mut(block.into());
            bytes_read += read;
        }
        Ok(bytes_read)
    }
}
