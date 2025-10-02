use eyre::{Context, bail};

use crate::protocol::packet::{CConfigurationPacket, CLoginPacket, ProtocolWrite, SLoginPacket};
use crate::server::conn::{Connection, ConnectionState};
use crate::server::player_profile::PlayerProfile;
use crate::types::Identifier;

impl Connection {
    pub fn handle_login_packet(&mut self, packet: SLoginPacket) -> crate::error::Result<()> {
        match packet {
            SLoginPacket::LoginStart { name, player_uuid } => {
                log::info!("{} ({}) wants to log in", name, player_uuid);

                self.player_profile = Some(PlayerProfile::new(player_uuid, name));

                let packet = self.server.read().crypt_keys().generate_encryption_request_packet();
                self.send_packet(packet)?;
            }
            SLoginPacket::EncryptionResponse { shared_secret, verify_token } => {
                {
                    let crypt_keys = &self.server.read().crypt_keys;
                    if !crypt_keys.verify_token(&verify_token).expect("should verify token") {
                        bail!("verification tokens are not the same");
                    }
                }

                self.enable_encryption(&shared_secret).wrap_err("failed to enable encryption")?;

                self.enable_compression().wrap_err("failed to enable compression")?;

                let player_profile = self.player_profile();
                self.send_packet(CLoginPacket::LoginSuccess {
                    uuid: player_profile.uuid().clone(),
                    username: player_profile.username().to_string(),
                    properties: player_profile.properties().clone(),
                })?;
            }
            SLoginPacket::LoginPluginResponse { .. } => todo!(),
            SLoginPacket::LoginAcknowledged => {
                log::debug!("login acknowledged");

                self.send_brand(crate::BRAND)?;

                self.state = ConnectionState::Configuration;
                self.finish_configuration()?;
            }
            SLoginPacket::CookieResponse { .. } => todo!(),
        }

        Ok(())
    }

    fn send_brand(&mut self, brand: &str) -> crate::error::Result<()> {
        let mut data = Vec::new();
        ProtocolWrite::write_all(brand, &mut data)?;

        self.send_packet(CConfigurationPacket::PluginMessage {
            channel: Identifier::new("minecraft", "brand")?,
            data,
        })?;

        Ok(())
    }

    fn finish_configuration(&mut self) -> crate::error::Result<()> {
        self.send_packet(CConfigurationPacket::FinishConfiguration)?;
        log::debug!("configuration finished");
        Ok(())
    }
}
