use std::io;
use std::path::PathBuf;

use tokio::net::TcpListener;

use crate::server::connection::Connection;
use crate::server::folder::ServerFolder;

pub mod connection;
pub mod folder;
pub mod player;
pub mod player_list;

pub struct Server {
    server_folder: ServerFolder,
}

impl Server {
    pub async fn new(server_folder_path: PathBuf) -> io::Result<Self> {
        let server_folder = ServerFolder::new(server_folder_path).await?;

        Ok(Self { server_folder })
    }

    pub fn server_folder(&self) -> &ServerFolder {
        &self.server_folder
    }

    pub async fn start(self) -> io::Result<()> {
        let listener = TcpListener::bind("0.0.0.0:25565").await?;

        loop {
            let (socket, addr) = listener.accept().await?;

            tokio::spawn(async move {
                let connection = Connection::new(socket, addr);
                if let Err(err) = connection.start().await {
                    log::error!("connection error: {}", err)
                };
            });
        }
    }
}
