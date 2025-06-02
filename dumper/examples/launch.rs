use std::error::Error;

use clap::Parser;
use dumper::config::{cli::VmmCliConfig, TryIntoVmm};

fn main() -> Result<(), Box<dyn Error>> {
    let config = VmmCliConfig::parse();
    let mut vmm = config.try_into_vmm()?;
    vmm.run()?; // Assuming `try_into` is the correct method to use here
    Ok(())
}
