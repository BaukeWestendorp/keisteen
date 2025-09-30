use craft::server::conn::ConnectionManager;
use craft::server::{Server, ServerHandle};

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let handle = ServerHandle::new(Server::new());
    ConnectionManager::new(handle).bind("127.0.0.1:25565").expect("should start server");
}
