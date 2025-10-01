use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::thread;

use eyre::{Context, bail};

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

    pub fn bind<A: ToSocketAddrs>(self, addr: A) -> crate::error::Result<()> {
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
    pub fn new(stream: TcpStream, server: ServerHandle) -> crate::error::Result<Self> {
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

    pub fn spawn(self) {
        let peer_address = self
            .writer
            .writer()
            .peer_addr()
            .map(|a| a.to_string())
            .unwrap_or("<no peer address>".to_string());

        thread::Builder::new()
            .name(format!("connection [{}]", peer_address))
            .spawn(move || {
                tracing::info!("new connection: {}", peer_address);
                self.run().unwrap()
            })
            .expect("should create thread");
    }

    fn run(mut self) -> eyre::Result<()> {
        fn is_connection_closed(err: &io::Error) -> bool {
            matches!(err.kind(), io::ErrorKind::BrokenPipe | io::ErrorKind::ConnectionReset)
        }

        loop {
            tracing::trace!("waiting for next packet in {:?} state...", self.state);

            let packet = match self.read_raw_packet() {
                Ok(packet) => packet,
                Err(err) if is_connection_closed(&err) => break,
                Err(err) => bail!(err),
            };

            if let Err(err) = self.handle_raw_packet(packet) {
                if let Some(io_err) = err.downcast_ref::<io::Error>() {
                    if is_connection_closed(io_err) {
                        break;
                    }
                }

                bail!(err);
            }
        }

        Ok(())
    }

    fn handle_raw_packet(&mut self, packet: RawPacket) -> crate::error::Result<()> {
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
                self.handle_configuration_packet(packet)?;
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
        let len = self.read_varint()?;
        let packet_id = self.read_varint()?;
        let data_len = (len.raw() as usize).saturating_sub(packet_id.len());
        let data = self.read_bytes(data_len)?;
        tracing::trace!(
            "received packet with packet id {:#04x} in state {:?}",
            packet_id.raw(),
            self.state
        );
        Ok(RawPacket { packet_id, data: PacketData::from(data) })
    }

    fn write_raw_packet(&mut self, packet: impl Into<RawPacket>) -> crate::error::Result<()> {
        let packet = packet.into();

        tracing::trace!(
            "sending packet with packet id {:#04x} in state {:?}",
            packet.packet_id.raw(),
            self.state
        );

        self.write_varint(VarInt::new(packet.length() as i32))?;
        self.write_varint(packet.packet_id)?;
        self.write_bytes(packet.data.bytes())?;

        tracing::trace!("sent packet");
        Ok(())
    }

    fn read_varint(&mut self) -> io::Result<VarInt> {
        Ok(VarInt::from_reader(&mut self.reader)?)
    }

    fn write_varint(&mut self, value: VarInt) -> io::Result<()> {
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
