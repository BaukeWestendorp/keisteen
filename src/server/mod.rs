use std::sync::{Arc, Mutex};

use rsa::Pkcs1v15Encrypt;
use rsa::traits::PublicKeyParts;

pub mod conn;
mod crypt;

pub struct Server {
    public_key_der: Vec<u8>,
    private_key: rsa::RsaPrivateKey,
    verify_token: [u8; 4],
}

impl Server {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let private_key =
            rsa::RsaPrivateKey::new(&mut rng, 1024).expect("failed to generate a key");

        Self {
            public_key_der: rsa_der::public_key_to_der(
                &private_key.n().to_bytes_be(),
                &private_key.e().to_bytes_be(),
            ),
            private_key,
            verify_token: rand::random(),
        }
    }

    fn verify_encryption_response(&self, verify_token: &[u8]) -> bool {
        let private_key = self.private_key.clone();
        let verify_token = private_key.decrypt(Pkcs1v15Encrypt::default(), &verify_token).unwrap();
        verify_token == self.verify_token
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
