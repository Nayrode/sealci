use clap::Parser;
use sealcid::{
    common::error::Error,
    server::{cli::Cli, config::GlobalConfig, daemon::Daemon},
};
use tokio::signal;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    let sealcid_config = Cli::parse();
    let global_config = GlobalConfig::default();
    let deamon = Daemon::new(global_config).await?;

    let server = deamon
        .start(sealcid_config.port);

    tokio::select! {
        res = server => {
            res.map_err(Error::StartGrpcError)?;
        }
        _ = signal::ctrl_c() => {
            println!("Received Ctrl+C, shutting down.");
        }
    }

    Ok(())
}
