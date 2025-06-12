mod app;
mod common;
mod config;
mod constants;
mod controller;
mod error;
mod event_listener;
mod external_api;
mod github;
mod service;

use clap::Parser;
use config::Config;
use tokio;

#[tokio::main]
async fn main() -> Result<(), error::Error> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();
    let config = Config::parse();
    let app = app::App::init(config.clone());
    app.await?.start().await?.await.expect("REASON");
    Ok(())
}
