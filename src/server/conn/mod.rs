use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::thread;

use crate::error::CraftError;
use crate::server::crypt::{DecryptionStream, EncryptionStream};

use crate::protocol::packet::{
    PacketData, RawPacket, SConfigurationPacket, SHandshakingPacket, SLoginPacket, SStatusPacket,
};
use crate::server::ServerHandle;
use crate::server::player_profile::PlayerProfile;
use crate::types::VarInt;

mod config;
mod handshaking;
mod login;
mod status;

pub struct ConnectionManager {
    server: ServerHandle,
}

impl ConnectionManager {
    pub fn new(server: ServerHandle) -> Self {
        Self { server }
    }

    pub fn bind<A: ToSocketAddrs>(self, addr: A) -> Result<(), CraftError> {
        let listener = TcpListener::bind(addr)?;
        tracing::info!("started listening on {}", listener.local_addr().unwrap());

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    Connection::new(stream, self.server.clone())?.spawn();
                }
                Err(err) => {
                    tracing::error!("failed to accept incoming connection: {err}")
                }
            }
        }

        Ok(())
    }
}

pub struct Connection {
    server: ServerHandle,

    state: ConnectionState,

    writer: EncryptionStream<TcpStream>,
    reader: DecryptionStream<TcpStream>,

    player_profile: Option<PlayerProfile>,
}

impl Connection {
    pub fn new(stream: TcpStream, server: ServerHandle) -> Result<Self, CraftError> {
        Ok(Self {
            server,

            state: ConnectionState::Handshaking,

            writer: EncryptionStream::new(stream.try_clone()?),
            reader: DecryptionStream::new(stream),

            player_profile: None,
        })
    }

    pub fn player_profile(&self) -> &PlayerProfile {
        self.player_profile.as_ref().expect("player should have been initialized at login")
    }

    pub fn spawn(mut self) {
        let peer_address = self
            .writer
            .writer()
            .peer_addr()
            .map(|a| a.to_string())
            .unwrap_or("<unknown peer address>".to_string());

        thread::Builder::new()
            .name(format!("connection [{}]", peer_address))
            .spawn::<_, Result<(), CraftError>>(move || {
                tracing::info!("new connection: {}", peer_address);

                loop {
                    tracing::trace!("waiting for next packet in {:?} state...", self.state);
                    let packet = self.read_raw_packet()?;
                    self.handle_raw_packet(packet)?;
                }
            })
            .expect("should create thread");
    }

    fn handle_raw_packet(&mut self, packet: RawPacket) -> Result<(), CraftError> {
        match self.state {
            ConnectionState::Handshaking => {
                let packet = SHandshakingPacket::try_from(packet)?;
                self.handle_handshaking_packet(packet)?;
            }
            ConnectionState::Status => {
                let packet = SStatusPacket::try_from(packet)?;
                self.handle_status_packet(packet)?;
            }
            ConnectionState::Transfer => {
                todo!();
            }
            ConnectionState::Login => {
                let packet = SLoginPacket::try_from(packet)?;
                self.handle_login_packet(packet)?;
            }
            ConnectionState::Configuration => {
                let packet = SConfigurationPacket::try_from(packet)?;
                self.handle_configuration_packet(packet)?
            }
            ConnectionState::Play => {
                todo!();
            }
        }

        Ok(())
    }

    fn enable_encryption(&mut self, shared_secret: &[u8]) -> io::Result<()> {
        let shared_secret =
            self.server.read().crypt_keys().decrypt(shared_secret).expect("should decrypt secret");

        self.writer.enable_encryption(&shared_secret);
        self.reader.enable_encryption(&shared_secret);
        Ok(())
    }

    fn read_raw_packet(&mut self) -> io::Result<RawPacket> {
        let len = self.read_var_int()?;
        let packet_id = match len.raw() {
            0 => VarInt::new(0x00),
            _ => self.read_var_int()?,
        };
        let data_len = (len.raw() as usize).saturating_sub(packet_id.len());
        let data = self.read_bytes(data_len)?;
        tracing::trace!("received packet");
        Ok(RawPacket { packet_id, data: PacketData::from(data) })
    }

    fn write_raw_packet(&mut self, packet: impl Into<RawPacket>) -> io::Result<()> {
        tracing::trace!("sending packet...");
        let packet = packet.into();
        self.write_var_int(VarInt::new(packet.length() as i32))?;
        self.write_var_int(packet.packet_id)?;
        self.write_bytes(packet.data.bytes())?;
        tracing::trace!("sent packet");
        Ok(())
    }

    fn read_var_int(&mut self) -> io::Result<VarInt> {
        Ok(VarInt::from_reader(&mut self.reader)?)
    }

    fn write_var_int(&mut self, value: VarInt) -> io::Result<()> {
        value.to_writer(&mut self.writer)?;
        Ok(())
    }

    fn read_bytes(&mut self, len: usize) -> io::Result<Vec<u8>> {
        let mut buffer = vec![0u8; len];
        self.reader.read_exact(&mut buffer)?;
        Ok(buffer)
    }

    fn write_bytes(&mut self, bytes: &[u8]) -> io::Result<()> {
        self.writer.write_all(bytes)?;
        Ok(())
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
