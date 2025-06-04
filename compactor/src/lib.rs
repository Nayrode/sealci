use std::io::Cursor;

use dumper::vmm::VMM;

use crate::{cli::Cli, error::Error, kernel::VMLINUX};

pub mod cli;
pub mod error;
pub mod kernel;

pub struct Compactor {
    vmm: VMM,
    interactive: bool,
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
        let vmm_config = dumper::config::VmmConfig {
            mem_size_mb: config.mem_size_mb,
            num_vcpus: config.num_vcpus,
            kernel: kernel,
            initramfs: initramfs,
            enable_network: true,
            network_mac: "".to_string(),
            tap_interface_name: config.tap_interface_name.clone(),
        };
        let vmm = vmm_config
            .try_into_vmm()
            .await
            .map_err(Error::DumperError)?;
        Ok(Self {
            vmm,
            interactive: config.interactive,
        })
    }

    pub fn run(&mut self) -> Result<(), Error> {
        self.vmm.run(self.interactive).map_err(Error::DumperError)
    }
}
