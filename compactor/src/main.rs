use clap::Parser;
use compactor::{Compactor, config::Config, error::Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = Config::parse();
    let mut compactor = Compactor::new(config).await?;
    compactor.run();
    
    Ok(())
}
