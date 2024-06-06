mod config;
mod server;

use axum::handler::Handler;
use config::Config;
use std::fs::File;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let config: Config =
        serde_yaml::from_reader(File::open("config.yaml").expect("cannot open config file"))
            .expect("invalid config sfile");

    let listener = tokio::net::TcpListener::bind(config.bind_address)
        .await
        .unwrap();
    let handler = server::entry.with_state(server::make_state(config.gateways));
    axum::serve(listener, handler).await.unwrap();
}
