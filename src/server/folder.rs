use std::net::Ipv4Addr;
use std::path::PathBuf;

use tokio::io;

const PROPERTIES_FILE_PATH: &str = "properties.toml";
const CONFIG_FILE_PATH: &str = "config.toml";

pub struct ServerFolder {
    path: PathBuf,

    properties: ServerProperties,
    config: ServerConfig,
}

impl ServerFolder {
    pub async fn new(path: PathBuf) -> io::Result<Self> {
        let properties = ServerProperties::load_from_file(&path.join(PROPERTIES_FILE_PATH)).await;
        let properties = match properties {
            Ok(props) => props,
            Err(err) if err.kind() == io::ErrorKind::NotFound => {
                log::warn!("properties file not found, using defaults");
                ServerProperties::default()
            }
            Err(err) => return Err(err),
        };

        let config = ServerConfig::load_from_file(&path.join(CONFIG_FILE_PATH)).await;
        let config = match config {
            Ok(props) => props,
            Err(err) if err.kind() == io::ErrorKind::NotFound => {
                log::warn!("config file not found, using defaults");
                ServerConfig::default()
            }
            Err(err) => return Err(err),
        };

        Ok(Self { path, properties, config })
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn properties(&self) -> &ServerProperties {
        &self.properties
    }

    pub fn config(&self) -> &ServerConfig {
        &self.config
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ServerProperties {
    max_players: u32,
    motd: String,
    server_ip: Ipv4Addr,
    server_port: u16,
}

impl ServerProperties {
    async fn load_from_file(path: &PathBuf) -> io::Result<Self> {
        let content = tokio::fs::read_to_string(path).await?;
        toml::from_str(&content).map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, format!("invalid properties file: {}", e))
        })
    }

    pub fn max_players(&self) -> u32 {
        self.max_players
    }

    pub fn motd(&self) -> &str {
        &self.motd
    }

    pub fn server_ip(&self) -> Ipv4Addr {
        self.server_ip
    }

    pub fn server_port(&self) -> u16 {
        self.server_port
    }
}

impl Default for ServerProperties {
    fn default() -> Self {
        Self {
            max_players: 20,
            motd: "A Keisteen Minecraft Server".to_string(),
            server_ip: Ipv4Addr::UNSPECIFIED,
            server_port: 25565,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ServerConfig {
    enfores_secure_chat: bool,
}

impl ServerConfig {
    async fn load_from_file(path: &PathBuf) -> io::Result<Self> {
        let content = tokio::fs::read_to_string(path).await?;
        toml::from_str(&content).map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, format!("invalid config file: {}", e))
        })
    }

    pub fn enfores_secure_chat(&self) -> bool {
        self.enfores_secure_chat
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self { enfores_secure_chat: false }
    }
}
