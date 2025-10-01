use eyre::bail;

use crate::protocol::packet::{CLoginPacket, SLoginPacket};
use crate::server::conn::{Connection, ConnectionState};
use crate::server::player_profile::PlayerProfile;

impl Connection {
    pub fn handle_login_packet(&mut self, packet: SLoginPacket) -> crate::error::Result<()> {
        match packet {
            SLoginPacket::LoginStart { name, player_uuid } => {
                tracing::info!("{} ({}) wants to log in", name, player_uuid);

                self.player_profile = Some(PlayerProfile::new(player_uuid, name));

                let packet = self.server.read().crypt_keys().generate_encryption_request_packet();
                self.write_raw_packet(packet)?;
            }
            SLoginPacket::EncryptionResponse { shared_secret, verify_token } => {
                {
                    let crypt_keys = &self.server.read().crypt_keys;
                    if !crypt_keys.verify_token(&verify_token).expect("should verify token") {
                        bail!("verification tokens are not the same");
                    }
                }

                self.enable_encryption(&shared_secret)?;
                tracing::debug!("encryption enabled");

                let player_profile = self.player_profile();
                self.write_raw_packet(CLoginPacket::LoginSuccess {
                    uuid: player_profile.uuid().clone(),
                    username: player_profile.username().to_string(),
                    properties: player_profile.properties().clone(),
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
}
