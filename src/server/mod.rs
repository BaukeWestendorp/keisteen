use std::io;
use std::net::{TcpListener, ToSocketAddrs};

use connection::Connection;

pub mod connection;
mod crypt;

pub struct Server {}

impl Server {
    pub fn bind<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        tracing::info!("server starting...");
        let listener = TcpListener::bind(addr)?;
        tracing::info!("server started");

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    Connection::spawn(stream);
                }
                Err(err) => {
                    tracing::error!("failed to accept incoming connection: {err}")
                }
            }
        }

        Ok(Self {})
    }
}
