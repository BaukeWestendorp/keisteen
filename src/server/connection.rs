use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::thread;

use rsa::Pkcs1v15Encrypt;
use rsa::traits::PublicKeyParts;
use uuid::Uuid;

use crate::error::CraftError;
use crate::server::crypt::{DecryptionStream, EncryptionStream};

use super::packet::CStatusPacket;
use crate::server::packet::{
    CConfigurationPacket, CLoginPacket, PacketData, RawPacket, SConfigurationPacket,
    SHandshakingPacket, SLoginPacket, SStatusPacket,
};
use crate::types::VarInt;

pub struct Connection {
    state: ConnectionState,

    public_key_der: Vec<u8>,
    private_key: rsa::RsaPrivateKey,
    verify_token: [u8; 4],

    writer: EncryptionStream<TcpStream>,
    reader: DecryptionStream<TcpStream>,

    username: String,
    uuid: Uuid,
}

impl Connection {
    pub fn spawn(stream: TcpStream) {
        thread::Builder::new()
            .name("connection".to_string())
            .spawn::<_, Result<(), CraftError>>(move || {
                tracing::info!("new connection: {}", {
                    stream
                        .peer_addr()
                        .map(|a| a.to_string())
                        .unwrap_or("<unknown peer address>".to_string())
                });

                let mut rng = rand::thread_rng();
                let private_key =
                    rsa::RsaPrivateKey::new(&mut rng, 1024).expect("failed to generate a key");

                let mut this = Self {
                    state: ConnectionState::Handshaking,

                    // TODO: Properly implement encryption
                    public_key_der: rsa_der::public_key_to_der(
                        &private_key.n().to_bytes_be(),
                        &private_key.e().to_bytes_be(),
                    ),
                    private_key,
                    verify_token: rand::random(),

                    writer: EncryptionStream::new(stream.try_clone()?),
                    reader: DecryptionStream::new(stream),

                    username: "".to_string(),
                    uuid: Uuid::default(),
                };

                loop {
                    tracing::trace!("waiting for next packet in {:?} state...", this.state);
                    let packet = this.read_raw_packet()?;
                    this.handle_raw_packet(packet)?;
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
            ConnectionState::Login => {
                let packet = SLoginPacket::try_from(packet)?;
                self.handle_login_packet(packet)?;
            }
            ConnectionState::Transfer => todo!(),
            ConnectionState::Configuration => {
                let packet = SConfigurationPacket::try_from(packet)?;
                self.handle_configuration_packet(packet)?
            }
            ConnectionState::Play => todo!(),
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
                let json_response = r#"{"version":{"name":"1.21.8","protocol":772},"players":{"max":20,"online":0,"sample":[]},"description":{"text":"\u00a74!!\u00a76\u00a7l craft\u00a74 !!"},"enforcesSecureChat":false}"#.to_string();
                self.write_raw_packet(CStatusPacket::StatusResponse { json_response })?;
            }
            SStatusPacket::PingRequest { timestamp } => {
                self.write_raw_packet(CStatusPacket::PongResponse { timestamp })?;
            }
        }

        Ok(())
    }

    fn handle_login_packet(&mut self, packet: SLoginPacket) -> Result<(), CraftError> {
        match packet {
            SLoginPacket::LoginStart { name, player_uuid } => {
                tracing::info!("{} ({}) wants to log in", name, player_uuid);

                self.username = name;
                self.uuid = player_uuid;

                self.write_raw_packet(CLoginPacket::EncryptionRequest {
                    server_id: "".to_string(),
                    public_key: self.public_key_der.clone(),
                    verify_token: self.verify_token.to_vec(),
                    // TODO:
                    should_authenticate: false,
                })?;
            }
            SLoginPacket::EncryptionResponse { shared_secret, verify_token } => {
                let shared_secret =
                    self.private_key.decrypt(Pkcs1v15Encrypt::default(), &shared_secret).unwrap();
                let verify_token =
                    self.private_key.decrypt(Pkcs1v15Encrypt::default(), &verify_token).unwrap();

                if verify_token != self.verify_token {
                    return Err(CraftError::AuthenticationFailed);
                }

                self.enable_encryption(&shared_secret)?;
                tracing::debug!("encryption enabled");

                self.write_raw_packet(CLoginPacket::LoginSuccess {
                    uuid: self.uuid,
                    username: self.username.clone(),
                    properties: (),
                })?;
            }
            SLoginPacket::LoginPluginResponse { .. } => todo!(),
            SLoginPacket::LoginAcknowledged => {
                self.state = ConnectionState::Configuration;
                tracing::debug!("login acknowledged");
            }
            SLoginPacket::CookieResponse { .. } => todo!(),
        }

        Ok(())
    }

    fn handle_configuration_packet(
        &mut self,
        packet: SConfigurationPacket,
    ) -> Result<(), CraftError> {
        match packet {
            SConfigurationPacket::ClientInformation { .. } => {
                // TODO: Do something with client information.
                self.write_raw_packet(CConfigurationPacket::FinishConfiguration)?;
            }
            SConfigurationPacket::CookieResponse => todo!(),
            SConfigurationPacket::PluginMessage { channel, data } => {
                tracing::debug!("received channel message on channel '{channel}': {data:?}");
            }
            SConfigurationPacket::AcknowledgeFinishConfiguration => {
                tracing::debug!("configuration acknowledged");
                self.state = ConnectionState::Play;
            }
            SConfigurationPacket::KeepAlive => todo!(),
            SConfigurationPacket::Pong => todo!(),
            SConfigurationPacket::ResourcePackResponse => todo!(),
            SConfigurationPacket::KnownPacks => todo!(),
            SConfigurationPacket::CustomClickAction => todo!(),
        }

        Ok(())
    }

    fn enable_encryption(&mut self, shared_secret: &[u8]) -> io::Result<()> {
        self.writer.enable_encryption(&shared_secret);
        self.reader.enable_encryption(&shared_secret);
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
        Ok(VarInt::from_reader(&mut self.reader)?)
    }

    fn write_var_int(&mut self, value: VarInt) -> io::Result<()> {
        value.to_writer(&mut self.writer)?;
        Ok(())
    }

    fn write_bytes(&mut self, bytes: &[u8]) -> io::Result<()> {
        self.writer.write_all(bytes)?;
        Ok(())
    }

    fn read_bytes(&mut self, len: usize) -> io::Result<Vec<u8>> {
        let mut buffer = vec![0u8; len];
        self.reader.read_exact(&mut buffer)?;
        Ok(buffer)
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
