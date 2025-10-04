use uuid::Uuid;

use crate::protocol::packet::PacketData;
use crate::protocol::packet::client::ClientboundPacket;

#[derive(Debug)]
pub struct LoginSuccess {
    pub uuid: Uuid,
    pub username: String,
    pub properties: (),
}

impl ClientboundPacket for LoginSuccess {
    const PACKET_ID: i32 = 0x02;

    fn encode(self, data: &mut PacketData) {
        data.write(self.uuid);
        data.write(self.username);
        data.write_prefixed(Vec::<()>::new()); // TODO: Write properties.
    }
}
