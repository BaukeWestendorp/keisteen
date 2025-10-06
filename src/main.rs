use std::path::PathBuf;

use clap::Parser;

use crate::mc::types::VarInt;
use crate::server::Server;

pub(crate) mod error;
pub(crate) mod mc;
pub(crate) mod server;

pub const BRAND: &str = "Keisteen";
pub const MC_VERSION: &str = "1.21.8";
pub const MC_PROTOCOL: VarInt = VarInt::new(772);

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

    if let Err(err) = Server::new(path).start().await {
        log::error!("{err}");
    }
}
