use agent::{app::App, config::Config, models::error::Error};
use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();
    let config = Config::parse();
    let mut app = App::init(config).await?;
    app.start().await?;
    Ok(())
}
