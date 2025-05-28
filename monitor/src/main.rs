mod app;
mod config;
mod constants;
mod controller;
mod event_listener;
mod external_api;
mod file_utils;
mod github;
mod error;
use clap::Parser;
use config::Config;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();
    let config = Config::parse();
    let app = app::App::new(config.clone());
    app.run().await?;
    Ok(())
}
