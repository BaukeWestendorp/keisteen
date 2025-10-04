use eyre::{Context, bail};

use crate::error::KeisteenResult;
use crate::protocol::packet::server::ServerboundPacket;
use crate::protocol::packet::{self, RawPacket};
use crate::server::conn::Connection;

#[derive(Debug)]
pub struct EncryptionResponse {
    pub shared_secret: Vec<u8>,
    pub verify_token: Vec<u8>,
}

impl ServerboundPacket for EncryptionResponse {
    const PACKET_ID: i32 = 0x01;

    fn decode(mut raw: RawPacket) -> KeisteenResult<Self> {
        Ok(Self {
            shared_secret: raw.data.read_prefixed()?,
            verify_token: raw.data.read_prefixed()?,
        })
    }

    fn handle(&self, conn: &mut Connection) -> KeisteenResult<()> {
        let server = conn.server().read();
        let crypt_keys = &server.crypt_keys();
        if !crypt_keys.verify_token(&self.verify_token).expect("should verify token") {
            bail!("verification tokens are not the same");
        }
        drop(server);

        conn.enable_encryption(&self.shared_secret).wrap_err("failed to enable encryption")?;
        conn.enable_compression().wrap_err("failed to enable compression")?;

        let player_profile = conn.player_profile();
        conn.send_packet(packet::client::login::LoginSuccess {
            uuid: player_profile.uuid().clone(),
            username: player_profile.username().to_string(),
            properties: player_profile.properties().clone(),
        })?;

        Ok(())
    }
}
