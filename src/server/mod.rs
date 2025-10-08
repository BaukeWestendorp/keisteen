use std::collections::HashMap;
use std::io;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use tokio::net::TcpListener;
use uuid::Uuid;

use crate::mc::registries::Registries;
use crate::server::connection::Connection;

pub mod connection;
pub mod folder;
pub mod game_profile;
pub mod player;
pub mod player_list;

pub use folder::*;
pub use game_profile::*;
pub use player::*;
pub use player_list::*;

#[derive(Clone)]
pub struct Server {
    server_folder: Arc<ServerFolder>,

    player_list: Arc<PlayerList>,
    game_profiles: Arc<Mutex<HashMap<Uuid, GameProfile>>>,
    registries: Arc<Registries>,
}

impl Server {
    pub async fn new(server_folder_path: PathBuf) -> io::Result<Self> {
        let server_folder = Arc::new(ServerFolder::new(server_folder_path).await?);

        let max_players = server_folder.properties().max_players();
        let player_list = Arc::new(PlayerList::new(max_players));

        Ok(Self {
            server_folder,
            player_list,
            game_profiles: Arc::new(Mutex::new(HashMap::new())),
            registries: Arc::new(Registries::load_from_assets()),
        })
    }

    pub fn server_folder(&self) -> &ServerFolder {
        &self.server_folder
    }

    pub fn player_list(&self) -> &PlayerList {
        &self.player_list
    }

    pub fn registries(&self) -> &Registries {
        &self.registries
    }

    pub fn game_profile(&self, uuid: &Uuid) -> Option<GameProfile> {
        self.game_profiles.lock().unwrap().get(uuid).cloned()
    }

    pub fn add_game_profile(&self, profile: GameProfile) {
        self.game_profiles.lock().unwrap().insert(profile.uuid, profile);
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
