mod config;
mod server;

use axum::handler::Handler;
use config::Config;
use std::{fs::File, io};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let config: Config = match File::open("config.yaml") {
        Err(err) if err.kind() == io::ErrorKind::NotFound => Config::default(),
        Err(err) => panic!("cannot open config file: {err}"),
        Ok(file) => serde_yaml::from_reader(file).expect("invalid config file"),
    };

    let listener = tokio::net::TcpListener::bind(config.bind_address)
        .await
        .unwrap();
    let handler = server::entry.with_state(server::make_state(config.gateways));
    axum::serve(listener, handler).await.unwrap();
}
