use std::io;
use std::path::PathBuf;

use tokio::net::TcpListener;

use crate::server::connection::Connection;

pub mod connection;
pub mod player;
pub mod player_list;

pub struct Server {
    _server_folder_path: PathBuf,
}

impl Server {
    pub fn new(_server_folder_path: PathBuf) -> Self {
        Self { _server_folder_path }
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
