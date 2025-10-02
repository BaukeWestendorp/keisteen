use crate::protocol::packet::{
    CConfigurationPacket, KnownPack, ProtocolWrite, SConfigurationPacket,
};
use crate::server::conn::{Connection, ConnectionState};
use crate::types::Identifier;

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
            SConfigurationPacket::KnownPacks { known_packs } => {
                log::debug!("client's known packs: {known_packs:?}");
                // TODO: Do something with known packs.
            }
            SConfigurationPacket::CustomClickAction => todo!(),
        }

        Ok(())
    }

    pub fn start_configuration(&mut self) -> crate::error::Result<()> {
        self.state = ConnectionState::Configuration;

        self.send_brand(crate::BRAND)?;
        // TODO: Send Feature Flags
        self.send_known_packs()?;

        self.finish_configuration()?;

        Ok(())
    }

    fn send_brand(&mut self, brand: &str) -> crate::error::Result<()> {
        let mut data = Vec::new();
        ProtocolWrite::write_all(brand, &mut data)?;

        self.send_packet(CConfigurationPacket::PluginMessage {
            channel: Identifier::new("minecraft", "brand")?,
            data,
        })?;

        Ok(())
    }

    fn send_known_packs(&mut self) -> crate::error::Result<()> {
        // TODO: Actually get known packs.
        let known_packs = vec![KnownPack {
            namespace: "minecraft".to_string(),
            id: "core".to_string(),
            version: crate::MC_VERSION.to_string(),
        }];

        self.send_packet(CConfigurationPacket::KnownPacks { known_packs })?;

        Ok(())
    }

    fn finish_configuration(&mut self) -> crate::error::Result<()> {
        self.send_packet(CConfigurationPacket::FinishConfiguration)?;

        log::debug!("configuration finished");

        Ok(())
    }
}
