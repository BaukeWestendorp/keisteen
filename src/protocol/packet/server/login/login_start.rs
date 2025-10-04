use uuid::Uuid;

use crate::error::KeisteenResult;
use crate::protocol::packet::RawPacket;
use crate::protocol::packet::server::ServerboundPacket;
use crate::server::conn::Connection;
use crate::server::player_profile::PlayerProfile;

#[derive(Debug)]
pub struct LoginStart {
    pub name: String,
    pub player_uuid: Uuid,
}

impl ServerboundPacket for LoginStart {
    const PACKET_ID: i32 = 0x00;

    fn decode(mut raw: RawPacket) -> KeisteenResult<Self> {
        Ok(Self { name: raw.data.read()?, player_uuid: raw.data.read()? })
    }

    fn handle(&self, conn: &mut Connection) -> KeisteenResult<()> {
        log::info!("{} ({}) wants to log in", self.name, self.player_uuid);

        conn.player_profile = Some(PlayerProfile::new(self.player_uuid, self.name.clone()));

        // TODO: Implement authentication with Mojang's servers.
        let should_authenticate = false;
        let packet = conn
            .server()
            .read()
            .crypt_keys()
            .generate_encryption_request_packet(should_authenticate);

        conn.send_packet(packet)?;

        Ok(())
    }
}
