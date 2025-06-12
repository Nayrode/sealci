use sealci_scheduler::{
    errors::Error,
    config::Config,
    app::App,
};

use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Initialize logger.
    tracing_subscriber::fmt::init();

    let config = Config::parse();
    let app = App::init(config)?;
    app.start().await?;

    Ok(())
}
