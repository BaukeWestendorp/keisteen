use crate::error::CraftError;
use crate::protocol::packet::{CLoginPacket, SLoginPacket};
use crate::server::conn::{Connection, ConnectionState};

impl Connection {
    pub fn handle_login_packet(&mut self, packet: SLoginPacket) -> Result<(), CraftError> {
        match packet {
            SLoginPacket::LoginStart { name, player_uuid } => {
                tracing::info!("{} ({}) wants to log in", name, player_uuid);

                self.username = name;
                self.uuid = player_uuid;

                let packet = self.server.read().crypt_keys().generate_encryption_request_packet();
                self.write_raw_packet(packet)?;
            }
            SLoginPacket::EncryptionResponse { shared_secret, verify_token } => {
                {
                    let crypt_keys = &self.server.read().crypt_keys;
                    if !crypt_keys.verify_token(&verify_token).expect("should verify token") {
                        return Err(CraftError::VerificationTokenMismatch);
                    }
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
}
