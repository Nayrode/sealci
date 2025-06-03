use std::io::Cursor;

use dumper::vmm::VMM;

use crate::{cli::Cli, error::Error, kernel::VMLINUX};

pub mod cli;
pub mod error;
pub mod kernel;

pub struct Compactor {
    vmm: VMM,
}

impl Compactor {
    pub async fn new(config: Cli) -> Result<Self, Error> {
        let kernel = Cursor::new(VMLINUX);
        let mut envs = Vec::new();
        for env in &config.env {
            envs.push(env.as_str());
        }
        let initramfs =
            dumplet::generate_initramfs_image(&config.image, Some(envs), config.transfer_files)
                .await
                .map_err(Error::DumpletError)?;
        let config = dumper::config::VmmConfig {
            mem_size_mb: 2048,
            num_vcpus: 2,
            kernel: kernel,
            initramfs: initramfs,
            enable_network: true,
            network_mac: "".to_string(),
            tap_interface_name: "tap0".to_string(),
        };
        let vmm = config.try_into_vmm().await.map_err(Error::DumperError)?;
        Ok(Self { vmm })
    }

    pub fn run(&mut self) -> Result<(), Error> {
        self.vmm.run().map_err(Error::DumperError)
    }
}
