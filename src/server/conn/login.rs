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

                let public_key = self.server.read().public_key_der.to_vec();
                let verify_token = self.server.read().verify_token.to_vec();
                self.write_raw_packet(CLoginPacket::EncryptionRequest {
                    server_id: "".to_string(),
                    public_key,
                    verify_token,
                    // TODO:
                    should_authenticate: false,
                })?;
            }
            SLoginPacket::EncryptionResponse { shared_secret, verify_token } => {
                if !self.server.read().verify_encryption_response(&verify_token) {
                    return Err(CraftError::EncryptionMismatch);
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
