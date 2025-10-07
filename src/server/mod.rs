use std::io;
use std::path::PathBuf;
use std::sync::Arc;

use tokio::net::TcpListener;

use crate::server::connection::Connection;
use crate::server::folder::ServerFolder;
use crate::server::player_list::PlayerList;

pub mod connection;
pub mod folder;
pub mod player;
pub mod player_list;

#[derive(Clone)]
pub struct Server {
    server_folder: Arc<ServerFolder>,

    player_list: Arc<PlayerList>,
}

impl Server {
    pub async fn new(server_folder_path: PathBuf) -> io::Result<Self> {
        let server_folder = Arc::new(ServerFolder::new(server_folder_path).await?);

        let max_players = server_folder.properties().max_players();
        let player_list = Arc::new(PlayerList::new(max_players));

        Ok(Self { server_folder, player_list })
    }

    pub fn server_folder(&self) -> &ServerFolder {
        &self.server_folder
    }

    pub fn player_list(&self) -> &PlayerList {
        &self.player_list
    }
}

pub async fn start(server: Server) -> io::Result<()> {
    let address = format!(
        "{}:{}",
        server.server_folder().properties().server_ip(),
        server.server_folder().properties().server_port()
    );

    let listener = TcpListener::bind(address).await?;

    loop {
        let (socket, addr) = listener.accept().await?;

        tokio::spawn({
            let server = server.clone();
            async move {
                let connection = Connection::new(server, socket, addr);
                if let Err(err) = connection.start().await {
                    log::error!("connection error: {}", err)
                };
            }
        });
    }
}
