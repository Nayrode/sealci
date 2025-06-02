use clap::Parser;
use dumper::{
    common::error::Error,
    config::{cli::VmmCliConfig, TryIntoVmm},
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    VmmCliConfig::parse().try_into_vmm().await?.run()?;
    Ok(())
}
