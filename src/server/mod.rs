use std::sync::{Arc, Mutex, MutexGuard};

use crate::protocol::registry::Registries;
use crate::server::crypt::CryptKeys;

pub mod conn;
mod crypt;
mod player_profile;

pub struct Server {
    crypt_keys: CryptKeys,
    registries: Registries,
}

impl Server {
    pub fn new() -> Self {
        Self { crypt_keys: CryptKeys::new(), registries: Registries::load_from_assets() }
    }

    pub fn crypt_keys(&self) -> &CryptKeys {
        &self.crypt_keys
    }

    pub fn registries(&self) -> &Registries {
        &self.registries
    }
}

#[derive(Clone)]
pub struct ServerHandle(Arc<Mutex<Server>>);

impl ServerHandle {
    pub fn new(server: Server) -> Self {
        Self(Arc::new(Mutex::new(server)))
    }

    pub fn read(&self) -> MutexGuard<'_, Server> {
        self.0.lock().expect("Server mutex poisoned")
    }
}
