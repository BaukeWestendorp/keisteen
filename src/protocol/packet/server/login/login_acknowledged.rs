use crate::error::KeisteenResult;
use crate::protocol::packet::known_pack::KnownPack;
use crate::protocol::packet::server::ServerboundPacket;
use crate::protocol::packet::{ProtocolWrite, RawPacket, client};
use crate::server::conn::{Connection, ConnectionState};
use crate::types::Identifier;

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
