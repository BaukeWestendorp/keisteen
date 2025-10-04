use std::path::PathBuf;

use clap::Parser;
use keisteen::server::conn::ConnectionManager;
use keisteen::server::folder::ServerFolder;
use keisteen::server::{Server, ServerHandle};

#[derive(Parser)]
#[command(name = "Keisteen Server")]
#[command(about = "An experimental Minecraft Server implementation", version = "0.0.1")]
struct Args {
    /// The path to the server folder.
    #[arg(long, default_value = "./")]
    path: PathBuf,
}

fn main() {
    env_logger::builder().format_timestamp(None).init();

    let Args { path } = Args::parse();

    let server_folder = ServerFolder::new(path).expect("should create server");
    let properties = &server_folder.config().properties();
    let address = format!("{}:{}", properties.server_ip, properties.server_port);

    let server = Server::new(server_folder);
    let handle = ServerHandle::new(server);

    ConnectionManager::new(handle).bind(address).expect("should start server");
}
