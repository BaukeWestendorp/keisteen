use crate::error::KeisteenResult;
use crate::protocol::packet::known_pack::KnownPack;
use crate::protocol::packet::server::ServerboundPacket;
use crate::protocol::packet::{self, ProtocolWrite, RawPacket, client};
use crate::server::conn::{Connection, ConnectionState};
use crate::server::player_profile::PlayerProfile;
use crate::types::{Identifier, VarInt};
use eyre::{Context, bail};
use uuid::Uuid;

pub fn handle_raw_packet(raw: RawPacket, conn: &mut Connection) -> KeisteenResult<()> {
    match raw.packet_id.raw() {
        CookieResponse::PACKET_ID => CookieResponse::decode(raw)?.handle(conn),
        EncryptionResponse::PACKET_ID => EncryptionResponse::decode(raw)?.handle(conn),
        LoginAcknowledged::PACKET_ID => LoginAcknowledged::decode(raw)?.handle(conn),
        PluginResponse::PACKET_ID => PluginResponse::decode(raw)?.handle(conn),
        Start::PACKET_ID => Start::decode(raw)?.handle(conn),
        _ => bail!("unknown login packet id: {}", raw.packet_id.raw()),
    }
}

#[derive(Debug)]
pub struct Start {
    pub name: String,
    pub player_uuid: Uuid,
}

impl ServerboundPacket for Start {
    const PACKET_ID: i32 = 0x00;

    fn decode(mut raw: RawPacket) -> KeisteenResult<Self> {
        Ok(Self { name: raw.data.read()?, player_uuid: raw.data.read()? })
    }

    fn handle(&self, conn: &mut Connection) -> KeisteenResult<()> {
        log::info!("{} ({}) wants to log in", self.name, self.player_uuid);

        conn.player_profile = Some(PlayerProfile::new(self.player_uuid, self.name.clone()));

        // TODO: Implement authentication with Mojang's servers.
        let should_authenticate = false;
        let packet = conn.server().read(|server| {
            server.crypt_keys().generate_encryption_request_packet(should_authenticate)
        });

        conn.send_packet(packet)?;

        Ok(())
    }
}

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
        conn.server().read(|server| {
            if !server.crypt_keys().verify_token(&self.verify_token).expect("should verify token") {
                bail!("verification tokens are not the same");
            }

            Ok(())
        })?;

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

pub struct PluginResponse {
    pub message_id: VarInt,
    pub data: Vec<u8>,
}

impl ServerboundPacket for PluginResponse {
    const PACKET_ID: i32 = 0x02;

    fn decode(_raw: RawPacket) -> KeisteenResult<Self> {
        todo!()
    }

    fn handle(&self, _conn: &mut Connection) -> KeisteenResult<()> {
        todo!()
    }
}

#[derive(Debug)]
pub struct LoginAcknowledged;

impl ServerboundPacket for LoginAcknowledged {
    const PACKET_ID: i32 = 0x03;

    fn decode(_raw: RawPacket) -> KeisteenResult<Self> {
        Ok(Self)
    }

    fn handle(&self, conn: &mut Connection) -> KeisteenResult<()> {
        log::debug!("login acknowledged");

        conn.state = ConnectionState::Config;

        self.send_brand_plugin_message_packet(crate::BRAND, conn)?;

        // TODO: Send Feature Flags

        self.send_known_packs_packet(conn)?;

        Ok(())
    }
}

impl LoginAcknowledged {
    fn send_brand_plugin_message_packet(
        &self,
        brand: &str,
        conn: &mut Connection,
    ) -> KeisteenResult<()> {
        let mut data = Vec::new();
        brand.write(&mut data)?;

        conn.send_packet(client::config::PluginMessage {
            channel: Identifier::new("minecraft", "brand")?,
            data,
        })?;

        Ok(())
    }

    fn send_known_packs_packet(&self, conn: &mut Connection) -> KeisteenResult<()> {
        // TODO: Actually synchronize known packs.
        let known_packs = vec![KnownPack {
            namespace: "minecraft".to_string(),
            id: "core".to_string(),
            version: crate::MC_VERSION.to_string(),
        }];

        conn.send_packet(client::config::KnownPacks { known_packs })?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct CookieResponse {
    pub key: Identifier,
    pub payload: Vec<u8>,
}

impl ServerboundPacket for CookieResponse {
    const PACKET_ID: i32 = 0x04;

    fn decode(_raw: RawPacket) -> KeisteenResult<Self> {
        todo!()
    }

    fn handle(&self, _conn: &mut Connection) -> KeisteenResult<()> {
        todo!()
    }
}
