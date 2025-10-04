use std::net::{Shutdown, TcpListener, TcpStream, ToSocketAddrs};
use std::{io, thread};

use crate::error::KeisteenResult;
use crate::mc::protocol::packet::RawPacket;
use crate::mc::protocol::packet::client::ClientboundPacket;
use crate::mc::text::text_component::TextComponent;
use crate::server::ServerHandle;
use crate::server::conn::packet::decoder::PacketDecoder;
use crate::server::conn::packet::encoder::PacketEncoder;
use crate::server::player_profile::PlayerProfile;

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

    pub(crate) state: ConnectionState,

    writer: PacketEncoder<TcpStream>,
    reader: PacketDecoder<TcpStream>,
    stream: TcpStream,

    pub(crate) player_profile: Option<PlayerProfile>,
}

impl Connection {
    pub fn new(stream: TcpStream, server: ServerHandle) -> KeisteenResult<Self> {
        Ok(Self {
            is_running: false,

            server,

            state: ConnectionState::Handshaking,

            writer: PacketEncoder::new(stream.try_clone()?),
            reader: PacketDecoder::new(stream.try_clone()?),
            stream,

            player_profile: None,
        })
    }

    pub fn server(&self) -> &ServerHandle {
        &self.server
    }

    pub fn player_profile(&self) -> &PlayerProfile {
        self.player_profile.as_ref().expect("player should have been initialized at login")
    }

    pub(crate) fn spawn(mut self) {
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
                        "thread '{}' in {:?} state stopped: {}",
                        thread::current().name().unwrap_or("<unnamed>"),
                        &self.state,
                        error
                    );
                }

                self.disconnect(None);
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

    pub fn disconnect(&mut self, reason: Option<TextComponent>) {
        if let Some(profile) = &self.player_profile {
            log::info!(
                "player '{}' disconnected{}",
                profile.username(),
                if let Some(reason) = reason { format!(": {}", reason) } else { String::new() }
            );
            self.server.update(|server| server.player_list_mut().remove_player(profile.uuid()));
        }

        self.player_profile = None;

        self.close();
    }

    fn close(&mut self) {
        let _ = self.stream.shutdown(Shutdown::Both);
        self.is_running = false;
    }

    pub(crate) fn enable_encryption(&mut self, shared_secret: &[u8]) -> KeisteenResult<()> {
        let shared_secret = self.server.read(|server| {
            server.crypt_keys().decrypt(shared_secret).expect("should decrypt secret")
        });

        self.writer.enable_encryption(&shared_secret)?;
        self.reader.enable_encryption(&shared_secret)?;

        log::debug!("encryption enabled");

        Ok(())
    }

    pub(crate) fn enable_compression(&mut self) -> KeisteenResult<()> {
        // TODO: Add the threshold to the config.
        let threshold = 256;
        // TODO: Add the level to the config.
        let level = 3;

        self.writer.enable_compression(threshold, level)?;
        self.reader.enable_compression()?;

        // TODO: Actually enable compression.
        // self.send_packet(CLoginPacket::SetCompression {
        //     threshold: VarInt::new(threshold as i32),
        // })?;

        log::debug!("compression enabled");

        Ok(())
    }

    pub(crate) fn send_packet<P: ClientboundPacket>(&mut self, packet: P) -> io::Result<()> {
        self.writer.write_packet(packet)
    }

    pub(crate) fn read_packet(&mut self) -> io::Result<RawPacket> {
        self.reader.read_packet()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ConnectionState {
    Handshaking,
    Status,
    Login,
    Transfer,
    Config,
    Play,
}
