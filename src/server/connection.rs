use std::net::SocketAddr;

use futures::StreamExt;
use tokio::net::TcpStream;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio_util::codec::{FramedRead, FramedWrite};

use crate::error::KeisteenResult;
use crate::mc::packet::{self, ServerboundRawPacket};
use crate::server::packet_codec::PacketCodec;

pub struct Connection {
    state: ConnectionState,

    framed_reader: FramedRead<OwnedReadHalf, PacketCodec>,
    framed_writer: FramedWrite<OwnedWriteHalf, PacketCodec>,
}

impl Connection {
    pub fn new(socket: TcpStream, addr: SocketAddr) -> Self {
        log::info!("new connection from {}", addr);
        let (reader, writer) = socket.into_split();
        let framed_reader = FramedRead::new(reader, PacketCodec);
        let framed_writer = FramedWrite::new(writer, PacketCodec);
        Self { state: ConnectionState::Handshake, framed_reader, framed_writer }
    }

    pub async fn start(mut self) -> KeisteenResult<()> {
        loop {
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

        Ok(())
    }

    async fn handle_raw_packet(&mut self, packet: ServerboundRawPacket) -> KeisteenResult<()> {
        log::info!("received packet: {:?}", packet);

        match self.state {
            ConnectionState::Handshake => {
                packet::server::handshake::handle_raw_packet(packet).await?;
            }
            ConnectionState::Status => {
                // packet::server::status::handle_raw_packet(packet).await?;
            }
            ConnectionState::Login => {
                // packet::server::login::handle_raw_packet(packet).await?;
            }
            ConnectionState::Config => {
                // packet::server::config::handle_raw_packet(packet).await?;
            }
            ConnectionState::Transfer => {
                // packet::server::transfer::handle_raw_packet(packet).await?;
            }
        }

        Ok(())
    }
}

pub enum ConnectionState {
    Handshake,
    Status,
    Login,
    Config,
    Transfer,
}
