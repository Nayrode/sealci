use std::error::Error;

use clap::Parser;
use dumper::{config::{cli::VmmCliConfig, TryIntoVmm}, vmm::VMM};

fn main() -> Result<(), Box<dyn Error>> {
    let config = VmmCliConfig::parse();
    let mut vmm: VMM = TryIntoVmm::try_into(config)?;
    vmm.run()?;
    Ok(())
}
