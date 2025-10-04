use std::net::{Shutdown, TcpListener, TcpStream, ToSocketAddrs};
use std::{io, thread};

use aes::cipher::KeyIvInit;
use eyre::bail;

use crate::error::KeisteenResult;
use crate::server::crypt::{DecryptionStream, EncryptionStream};

use crate::protocol::packet::{PacketData, RawPacket};
use crate::server::ServerHandle;
use crate::server::player_profile::PlayerProfile;
use crate::types::VarInt;

mod packet;

pub struct ConnectionManager {
    server: ServerHandle,
}

impl ConnectionManager {
    pub fn new(server: ServerHandle) -> Self {
        Self { server }
    }

    pub fn bind<A: ToSocketAddrs>(self, addr: A) -> KeisteenResult<()> {
        let listener = TcpListener::bind(addr)?;
        log::info!("started listening on {}", listener.local_addr().unwrap());

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    stream.set_nodelay(true)?;
                    Connection::new(stream, self.server.clone())?.spawn();
                }
                Err(err) => {
                    log::error!("failed to accept incoming connection: {err}")
                }
            }
        }

        Ok(())
    }
}

pub struct Connection {
    is_running: bool,

    server: ServerHandle,

    state: ConnectionState,

    writer: PacketWriter<TcpStream>,
    reader: PacketReader<TcpStream>,
    stream: TcpStream,

    player_profile: Option<PlayerProfile>,
}

impl Connection {
    pub fn new(stream: TcpStream, server: ServerHandle) -> KeisteenResult<Self> {
        Ok(Self {
            is_running: false,

            server,

            state: ConnectionState::Handshaking,

            writer: PacketWriter::new(stream.try_clone()?),
            reader: PacketReader::new(stream.try_clone()?),
            stream,

            player_profile: None,
        })
    }

    pub fn player_profile(&self) -> &PlayerProfile {
        self.player_profile.as_ref().expect("player should have been initialized at login")
    }

    pub fn spawn(mut self) {
        let peer_address = self
            .stream
            .peer_addr()
            .map(|a| a.to_string())
            .unwrap_or("<no peer address>".to_string());

        thread::Builder::new()
            .name(format!("connection [{}]", peer_address))
            .spawn(move || {
                log::info!("new connection: {}", peer_address);
                if let Err(error) = self.run() {
                    log::error!(
                        "thread '{}' in {:?} state panicked: {}",
                        thread::current().name().unwrap_or("<unnamed>"),
                        &self.state,
                        error
                    );
                    return;
                }
            })
            .expect("should create thread");
    }

    fn run(&mut self) -> KeisteenResult<()> {
        self.is_running = true;

        while self.is_running {
            log::trace!("waiting for next packet in {:?} state...", self.state);
            let packet = self.read_packet()?;
            self.handle_raw_packet(packet)?;
        }

        Ok(())
    }

    fn close(&mut self) -> io::Result<()> {
        self.stream.shutdown(Shutdown::Both)?;
        self.is_running = false;
        Ok(())
    }

    fn enable_encryption(&mut self, shared_secret: &[u8]) -> KeisteenResult<()> {
        let shared_secret =
            self.server.read().crypt_keys().decrypt(shared_secret).expect("should decrypt secret");

        self.writer.enable_encryption(&shared_secret)?;
        self.reader.enable_encryption(&shared_secret)?;

        log::debug!("encryption enabled");

        Ok(())
    }

    pub fn enable_compression(&mut self) -> KeisteenResult<()> {
        // // TODO: Add the threshold to the config.
        // let threshold = 256;
        // // TODO: Add the level to the config.
        // let level = 3;

        // self.writer.enable_compression(threshold, level)?;
        // self.reader.enable_compression()?;

        // self.send_packet(CLoginPacket::SetCompression {
        //     threshold: VarInt::new(threshold as i32),
        // })?;

        // log::debug!("compression enabled");

        Ok(())
    }

    pub fn send_packet(&mut self, packet: impl Into<RawPacket>) -> io::Result<()> {
        self.writer.write_packet(packet)
    }

    pub fn read_packet(&mut self) -> io::Result<RawPacket> {
        self.reader.read_packet()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ConnectionState {
    Handshaking,
    Status,
    Login,
    Transfer,
    Configuration,
    Play,
}

pub enum PacketWriter<W: io::Write> {
    Raw(Option<W>),
    Encrypted(Option<EncryptionStream<W>>),
    Compressed { writer: EncryptionStream<W>, threshold: u32, level: u32 },
}

impl<W: io::Write> PacketWriter<W> {
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

    pub fn write_packet(&mut self, packet: impl Into<RawPacket>) -> io::Result<()> {
        let packet = packet.into();

        let mut buf = Vec::new();
        packet.packet_id.to_writer(&mut buf)?;
        packet.data.to_writer(&mut buf)?;

        match self {
            PacketWriter::Raw(Some(writer)) => {
                packet.length().to_writer(writer)?;
                packet.packet_id.to_writer(writer)?;
                packet.data.to_writer(writer)?;
            }
            PacketWriter::Encrypted(Some(writer)) => {
                packet.length().to_writer(writer)?;
                packet.packet_id.to_writer(writer)?;
                packet.data.to_writer(writer)?;
            }
            PacketWriter::Compressed { writer, .. } => {
                // TODO: Implement compression.

                packet.length().to_writer(writer)?;
                packet.packet_id.to_writer(writer)?;
                packet.data.to_writer(writer)?;
            }
            _ => unreachable!(),
        }

        Ok(())
    }
}

pub enum PacketReader<R: io::Read> {
    Raw(Option<R>),
    Encrypted(Option<DecryptionStream<R>>),
    Compressed { reader: DecryptionStream<R> },
}

impl<R: io::Read> PacketReader<R> {
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
            PacketReader::Raw(Some(reader)) => {
                let length = VarInt::from_reader(reader)?;
                read_uncompresed(reader, length)?
            }
            PacketReader::Encrypted(Some(reader)) => {
                let length = VarInt::from_reader(reader)?;
                read_uncompresed(reader, length)?
            }
            PacketReader::Compressed { reader } => {
                // TODO: Implement compression.

                let length = VarInt::from_reader(reader)?;
                read_uncompresed(reader, length)?
            }
            _ => unreachable!(),
        };

        Ok(packet)
    }
}
