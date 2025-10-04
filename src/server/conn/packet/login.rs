use eyre::{Context, bail};

use crate::error::KeisteenResult;
use crate::protocol::packet::{CLoginPacket, SLoginPacket};
use crate::server::conn::Connection;
use crate::server::player_profile::PlayerProfile;

impl Connection {
    pub fn handle_login_packet(&mut self, packet: SLoginPacket) -> KeisteenResult<()> {
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
                self.start_config()?;
            }
            SLoginPacket::CookieResponse { .. } => todo!(),
        }

        Ok(())
    }
}
