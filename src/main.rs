use std::path::PathBuf;

use clap::Parser;
use keisteen::server::conn::ConnectionManager;
use keisteen::server::folder::ServerFolder;
use keisteen::server::{Server, ServerHandle};

const IP: &str = "0.0.0.0";

#[derive(Parser)]
#[command(name = "Keisteen Server")]
#[command(about = "An experimental Minecraft Server implementation", version = "0.0.1")]
struct Args {
    /// The path to the server folder.
    #[arg(long, default_value = "./")]
    path: PathBuf,

    /// The port to bind the server to.
    #[arg(long, default_value = "25565")]
    port: u16,
}

fn main() {
    env_logger::builder().format_timestamp(None).init();

    let Args { path, port } = Args::parse();

    let server_folder = ServerFolder::new(path).expect("should create server");
    let server = Server::new(server_folder);
    let handle = ServerHandle::new(server);

    let address = format!("{}:{}", IP, port);
    ConnectionManager::new(handle).bind(address).expect("should start server");
}
