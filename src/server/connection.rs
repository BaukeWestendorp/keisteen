use std::collections::BTreeMap;
use std::net::SocketAddr;
use std::ops::Deref;

use bytes::BytesMut;
use futures::{SinkExt, StreamExt};
use tokio::io::{self, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio_util::codec::{FramedRead, FramedWrite};

use crate::error::KeisteenResult;
use crate::mc::nbt;
use crate::mc::packet::client::ClientboundPacket;
use crate::mc::packet::client::config::RegistryDataEntry;
use crate::mc::packet::codec::PacketCodec;
use crate::mc::packet::{self, ClientboundRawPacket, KnownPack, ServerboundRawPacket};
use crate::mc::registries::{RegItem, Registry};
use crate::mc::resources::ResourceLocation;
use crate::mc::types::VarInt;
use crate::server::Server;

pub struct Connection {
    server: Server,

    running: bool,
    state: ConnectionState,

    framed_reader: FramedRead<OwnedReadHalf, PacketCodec>,
    framed_writer: FramedWrite<OwnedWriteHalf, PacketCodec>,
}

impl Connection {
    pub fn new(server: Server, socket: TcpStream, addr: SocketAddr) -> Self {
        socket.set_nodelay(true).expect("should set TCP_NODELAY");

        log::info!("new connection from {}", addr);
        let (reader, writer) = socket.into_split();
        let framed_reader = FramedRead::new(reader, PacketCodec);
        let framed_writer = FramedWrite::new(writer, PacketCodec);

        Self {
            server,

            running: false,
            state: ConnectionState::default(),

            framed_reader,
            framed_writer,
        }
    }

    pub async fn start(mut self) -> KeisteenResult<()> {
        self.running = true;

        while self.running {
            let raw_packet = match self.framed_reader.next().await {
                Some(Ok(packet)) => packet,
                Some(Err(e)) => {
                    log::error!("error reading packet: {}", e);
                    break;
                }
                None => {
                    log::info!("connection closed by peer");
                    break;
                }
            };

            self.handle_raw_packet(raw_packet).await?;
        }

        log::info!("closing connection");
        if let Err(err) = self.framed_writer.get_mut().shutdown().await {
            log::error!("error shutting down connection: {}", err);
        }

        Ok(())
    }

    pub async fn stop(&mut self) {
        self.running = false;
    }

    pub fn server(&self) -> &Server {
        &self.server
    }

    pub fn set_state(&mut self, state: ConnectionState) {
        log::trace!("changing state from {:?} to {:?}", self.state, state);
        self.state = state;
    }

    async fn handle_raw_packet(&mut self, packet: ServerboundRawPacket) -> KeisteenResult<()> {
        match self.state {
            ConnectionState::Handshake => {
                packet::server::handshake::handle_raw_packet(packet, self).await?;
            }
            ConnectionState::Status => {
                packet::server::status::handle_raw_packet(packet, self).await?;
            }
            ConnectionState::Login => {
                packet::server::login::handle_raw_packet(packet, self).await?;
            }
            ConnectionState::Config => {
                packet::server::config::handle_raw_packet(packet, self).await?;
            }
            ConnectionState::Transfer => {
                todo!("handling packets in Transfer state is not implemented yet");
            }
        }

        Ok(())
    }

    pub async fn send_packet<P: ClientboundPacket>(&mut self, packet: P) -> io::Result<()> {
        log::trace!(">>> {:?}", packet);

        let id = VarInt::new(P::PACKET_ID);
        let mut data = BytesMut::new();
        packet.encode_data(&mut data);

        let raw_packet = ClientboundRawPacket { id, data };

        self.framed_writer.send(raw_packet).await?;

        Ok(())
    }

    pub async fn synchronize_known_packs(&mut self) -> KeisteenResult<()> {
        log::trace!("synchronizing known packs");

        // TODO: Fill with actual known packs.
        let known_packs = vec![KnownPack {
            namespace: "minecraft".to_string(),
            id: "core".to_string(),
            version: crate::MC_VERSION.to_string(),
        }];

        self.send_packet(packet::client::config::KnownPacks { known_packs }).await?;

        Ok(())
    }

    pub async fn send_registry_data(&mut self) -> KeisteenResult<()> {
        log::trace!("sending registry data");

        let registries = self.server().registries();

        let packets = vec![
            create_packet(registries.banner_patterns())?,
            create_packet(registries.cat_variants())?,
            create_packet(registries.chat_types())?,
            create_packet(registries.chicken_variants())?,
            create_packet(registries.cow_variants())?,
            create_packet(registries.damage_types())?,
            // create_packet(registries.dialogs())?,
            create_packet(registries.dimension_types())?,
            create_packet(registries.frog_variants())?,
            create_packet(registries.painting_variants())?,
            create_packet(registries.pig_variants())?,
            create_packet(registries.trim_materials())?,
            create_packet(registries.trim_patterns())?,
            create_packet(registries.wolf_sound_variants())?,
            create_packet(registries.wolf_variants())?,
            // create_packet(registries.worldgen_biomes())?,
        ];

        for packet in packets {
            self.send_packet(packet).await?;
        }

        fn create_packet<R: Registry + serde::Serialize>(
            registry_entries: &BTreeMap<ResourceLocation, RegItem<R>>,
        ) -> KeisteenResult<packet::client::config::RegistryData> {
            Ok(packet::client::config::RegistryData {
                registry_id: R::identifier(),
                entries: {
                    let mut entries = Vec::new();
                    for (identifier, entry) in registry_entries {
                        let entry_nbt = nbt::to_nbt("", entry.deref())?;
                        entries.push(RegistryDataEntry {
                            entry_id: identifier.clone(),
                            data: Some(entry_nbt),
                        });
                    }
                    entries
                },
            })
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ConnectionState {
    #[default]
    Handshake,
    Status,
    Login,
    Config,
    Transfer,
}
