use craft::server::Server;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    Server::bind("127.0.0.1:25565").unwrap();
}
