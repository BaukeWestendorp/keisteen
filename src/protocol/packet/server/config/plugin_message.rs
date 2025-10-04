use crate::error::KeisteenResult;
use crate::protocol::packet::RawPacket;
use crate::protocol::packet::server::ServerboundPacket;
use crate::server::conn::Connection;
use crate::types::Identifier;

#[derive(Debug)]
pub struct PluginMessage {
    pub channel: Identifier,
    pub data: Vec<u8>,
}

impl ServerboundPacket for PluginMessage {
    const PACKET_ID: i32 = 0x02;

    fn decode(mut raw: RawPacket) -> KeisteenResult<Self> {
        let channel = raw.data.read()?;
        let data_len = raw.data.bytes().len();
        Ok(Self { channel, data: raw.data.read_predefined(data_len)? })
    }

    fn handle(&self, _conn: &mut Connection) -> KeisteenResult<()> {
        if self.channel.namespace() == "minecraft" && self.channel.value() == "brand" {
            let brand_string = str::from_utf8(&self.data)?;
            log::debug!("client brand: \"{}\"", brand_string);
        } else {
            log::debug!("received channel message on channel '{}': {:?}", self.channel, self.data);
        }

        Ok(())
    }
}
