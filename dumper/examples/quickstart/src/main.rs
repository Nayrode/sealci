use std::error::Error;

use dumper::{self, config::vmm::VmmConfig, vmm::VMM};

use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = VmmConfig::parse();
    let mut vmm = VMM::new()?;
    vmm.configure(config).await?;
    vmm.run()?;
    Ok(())
}
