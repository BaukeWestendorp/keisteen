use std::net::Ipv4Addr;
use std::path::PathBuf;

use eyre::Context;

use crate::error::KeisteenResult;

#[derive(Debug, Clone)]
pub struct ServerFolder {
    path: PathBuf,
    config: ServerConfig,
}

impl ServerFolder {
    pub fn new(path: PathBuf) -> KeisteenResult<Self> {
        Ok(Self {
            config: ServerConfig::load_from_path(&path)
                .wrap_err("failed to load config from path")?,
            path,
        })
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn config(&self) -> &ServerConfig {
        &self.config
    }
}

#[derive(Debug, Clone, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ServerConfig {
    properties: ServerProperties,
}

impl ServerConfig {
    const CONFIG_FILE_PATH: &str = "config.toml";

    fn load_from_path(server_folder_path: &PathBuf) -> KeisteenResult<Self> {
        let config_path = server_folder_path.join(Self::CONFIG_FILE_PATH);
        let config = if config_path.exists() {
            let config_data = std::fs::read_to_string(config_path)
                .wrap_err("failed to read server configuration file.")?;
            toml::from_str(&config_data).wrap_err("failed to parse server configuration file")?
        } else {
            log::warn!(
                "server configuration file does not exist at path {:?}, using default configuration.",
                config_path
            );
            Self::default()
        };

        Ok(config)
    }

    pub fn properties(&self) -> &ServerProperties {
        &self.properties
    }
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ServerProperties {
    pub max_players: i32,
    pub server_ip: Ipv4Addr,
    pub server_port: u16,
}

impl Default for ServerProperties {
    fn default() -> Self {
        Self { max_players: 0, server_ip: Ipv4Addr::UNSPECIFIED, server_port: 25565 }
    }
}
