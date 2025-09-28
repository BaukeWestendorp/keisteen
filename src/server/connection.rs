use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::thread;

use crate::error::CraftError;

use super::packet::CStatusPacket;
use crate::server::packet::{PacketData, RawPacket, SHandshakingPacket, SStatusPacket};
use crate::types::VarInt;

#[derive(Debug)]
pub struct Connection {
    stream: TcpStream,
    state: ConnectionState,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Self { stream, state: ConnectionState::Handshaking }
    }

    pub fn start(mut self) {
        thread::Builder::new()
            .name("connection".to_string())
            .spawn::<_, Result<(), CraftError>>(move || {
                tracing::info!("new connection: {}", {
                    self.stream
                        .peer_addr()
                        .map(|a| a.to_string())
                        .unwrap_or("<unknown peer address>".to_string())
                });

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
            ConnectionState::Login => todo!(),
            ConnectionState::Transfer => todo!(),
        }

        Ok(())
    }

    fn handle_handshaking_packet(&mut self, packet: SHandshakingPacket) -> Result<(), CraftError> {
        match packet {
            SHandshakingPacket::Handshake { intent, .. } => {
                self.state = intent;
            }
        }

        Ok(())
    }

    fn handle_status_packet(&mut self, packet: SStatusPacket) -> Result<(), CraftError> {
        match packet {
            SStatusPacket::StatusRequest => {
                // TODO: Populate this JSON with actual data.
                let json_response = r#"{"version":{"name":"1.21.8","protocol":772},"players":{"max":20,"online":420,"sample":[]},"description":{"text":"\u00a74!!\u00a76\u00a7l craft\u00a74 !!"},"enforcesSecureChat":false}"#.to_string();
                self.write_raw_packet(CStatusPacket::StatusResponse { json_response })?;
            }
            SStatusPacket::PingRequest { timestamp } => {
                self.write_raw_packet(CStatusPacket::PongResponse { timestamp })?;
            }
        }

        Ok(())
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

    fn read_var_int(&mut self) -> io::Result<VarInt> {
        Ok(VarInt::from_reader(&mut self.stream)?)
    }

    fn write_var_int(&mut self, value: VarInt) -> io::Result<()> {
        value.to_writer(&mut self.stream)?;
        Ok(())
    }

    fn write_bytes(&mut self, bytes: &[u8]) -> io::Result<()> {
        self.stream.write_all(bytes)?;
        Ok(())
    }

    fn read_bytes(&mut self, len: usize) -> io::Result<Vec<u8>> {
        let mut buffer = vec![0u8; len];
        self.stream.read_exact(&mut buffer)?;
        Ok(buffer)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ConnectionState {
    Handshaking,
    Status,
    Login,
    Transfer,
}
