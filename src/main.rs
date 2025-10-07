use std::path::PathBuf;

use clap::Parser;

use crate::mc::types::VarInt;
use crate::server::Server;

pub mod error;
pub mod mc;
pub mod server;

pub const BRAND: &str = "Keisteen";
pub const MC_VERSION: &str = "1.21.9";
pub const MC_PROTOCOL: VarInt = VarInt::new(773);

#[derive(Parser)]
#[command(name = "Keisteen Server")]
#[command(about = "An experimental Minecraft Server implementation", version = "0.0.1")]
struct Args {
    /// The path to the server folder.
    #[arg(long, default_value = "./")]
    path: PathBuf,
}

#[tokio::main]
async fn main() {
    env_logger::builder().format_timestamp(None).init();

    let Args { path } = Args::parse();

    let Ok(server_instance) = Server::new(path).await else {
        log::error!("failed to create server");
        return;
    };

    if let Err(err) = server::start(server_instance).await {
        log::error!("{err}");
    }
}
