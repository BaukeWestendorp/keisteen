use std::sync::{Arc, Mutex};

use crate::server::crypt::CryptKeys;

pub mod conn;
mod crypt;

pub struct Server {
    crypt_keys: CryptKeys,
}

impl Server {
    pub fn new() -> Self {
        Self { crypt_keys: CryptKeys::new() }
    }

    pub fn crypt_keys(&self) -> &CryptKeys {
        &self.crypt_keys
    }
}

#[derive(Clone)]
pub struct ServerHandle(Arc<Mutex<Server>>);

impl ServerHandle {
    pub fn new(server: Server) -> Self {
        Self(Arc::new(Mutex::new(server)))
    }

    pub fn read(&self) -> std::sync::MutexGuard<'_, Server> {
        self.0.lock().expect("Server mutex poisoned")
    }
}
