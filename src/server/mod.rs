use std::sync::{Arc, Mutex};

use crate::protocol::registry::Registries;
use crate::server::crypt::CryptKeys;
use crate::server::player_list::PlayerList;

pub mod conn;
pub mod entity;
pub mod player;
pub mod player_list;
pub mod player_profile;

mod crypt;

pub struct Server {
    crypt_keys: CryptKeys,
    registries: Registries,
    player_list: PlayerList,
}

impl Server {
    pub fn new() -> Self {
        // TODO: Get from server config.
        let max_players = 100;

        Self {
            crypt_keys: CryptKeys::new(),
            registries: Registries::load_from_assets(),
            player_list: PlayerList::new(max_players),
        }
    }

    pub fn crypt_keys(&self) -> &CryptKeys {
        &self.crypt_keys
    }

    pub fn registries(&self) -> &Registries {
        &self.registries
    }

    pub fn player_list(&self) -> &PlayerList {
        &self.player_list
    }

    pub fn player_list_mut(&mut self) -> &mut PlayerList {
        &mut self.player_list
    }
}

#[derive(Clone)]
pub struct ServerHandle(Arc<Mutex<Server>>);

impl ServerHandle {
    pub fn new(server: Server) -> Self {
        Self(Arc::new(Mutex::new(server)))
    }
    pub fn read<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&Server) -> R,
    {
        let server = self.0.lock().expect("server mutex poisoned");
        f(&server)
    }

    pub fn update<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut Server) -> R,
    {
        let mut server = self.0.lock().expect("server mutex poisoned");
        f(&mut server)
    }
}
