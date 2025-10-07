use std::path::PathBuf;

use tokio::io;

pub struct ServerFolder {
    path: PathBuf,

    properties: ServerProperties,
    config: ServerConfig,
}

impl ServerFolder {
    pub async fn new(path: PathBuf) -> io::Result<Self> {
        // TODO: Load from files.
        let properties = ServerProperties {};
        let config = ServerConfig {};

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

pub struct ServerProperties {}

pub struct ServerConfig {}
