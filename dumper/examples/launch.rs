use std::error::Error;

use clap::Parser;
use dumper::{config::vmm::VmmCliConfig, vmm::VMM};

fn main() -> Result<(), Box<dyn Error>> {
    let config = VmmCliConfig::parse();
    let mut vmm = VMM::try_from(config)?;
    vmm.run()?;
    Ok(())
}
