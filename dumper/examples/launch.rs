use clap::Parser;
use dumper::{
    common::error::Error,
    config::{cli::VmmCliConfig, TryIntoVmm},
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = VmmCliConfig::parse();
    let mut vmm = config.try_into_vmm().await?;
    vmm.run()?; // Assuming `try_into` is the correct method to use here
    Ok(())
}
