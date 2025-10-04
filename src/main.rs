use keisteen::server::conn::ConnectionManager;
use keisteen::server::{Server, ServerHandle};

fn main() {
    pretty_env_logger::init();

    let handle = ServerHandle::new(Server::new());
    ConnectionManager::new(handle).bind("127.0.0.1:25565").expect("should start server");
}
