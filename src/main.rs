use craft::server::conn::ConnectionManager;
use craft::server::{Server, ServerHandle};

fn main() {
    pretty_env_logger::init();

    let handle = ServerHandle::new(Server::new());
    ConnectionManager::new(handle).bind("127.0.0.1:25565").expect("should start server");
}
