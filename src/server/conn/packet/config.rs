use crate::protocol::packet::SConfigurationPacket;
use crate::server::conn::{Connection, ConnectionState};

impl Connection {
    pub fn handle_configuration_packet(
        &mut self,
        packet: SConfigurationPacket,
    ) -> crate::error::Result<()> {
        match packet {
            SConfigurationPacket::ClientInformation { .. } => {
                // TODO: Do something with client information.
            }
            SConfigurationPacket::CookieResponse => todo!(),
            SConfigurationPacket::PluginMessage { channel, data } => {
                if channel.namespace() == "minecraft" && channel.value() == "brand" {
                    let brand_string = str::from_utf8(&data)?;
                    log::debug!("client brand: \"{}\"", brand_string);
                } else {
                    log::debug!("received channel message on channel '{channel}': {data:?}");
                }
            }
            SConfigurationPacket::AcknowledgeFinishConfiguration => {
                log::debug!("configuration acknowledged");
                self.state = ConnectionState::Play;
            }
            SConfigurationPacket::KeepAlive => todo!(),
            SConfigurationPacket::Pong => todo!(),
            SConfigurationPacket::ResourcePackResponse => todo!(),
            SConfigurationPacket::KnownPacks => todo!(),
            SConfigurationPacket::CustomClickAction => todo!(),
        }

        Ok(())
    }
}
