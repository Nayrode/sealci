use std::{fs::File, path::PathBuf};

use clap::{self, arg, command, Parser};

use super::{TryIntoVmm, TryIntoVmmConfig, VmmConfig};
use crate::common::error::Error;

#[derive(Parser, TryIntoVmm)]
#[try_into_vmm_types(File)]
#[command(version, about, long_about = None)]
pub struct VmmCliConfig {
    #[arg(long, default_value = "512", help = "Memory size in megabytes")]
    pub mem_size_mb: u32,

    #[arg(long, help = "Path to the kernel image")]
    pub kernel_path: PathBuf,

    #[arg(long, default_value = "1", help = "Number of virtual CPUs")]
    pub num_vcpus: u8,

    #[arg(long, default_value = "true", help = "Enable virtio network device")]
    pub enable_network: bool,

    #[arg(
        long,
        default_value = "52:54:00:12:34:56",
        help = "MAC address for the virtio network device"
    )]
    pub network_mac: String,

    #[arg(long, help = "The path to the initramfs folder")]
    pub initramfs_path: PathBuf,
}

impl VmmCliConfig {
    pub fn new(
        mem_size_mb: u32,
        kernel_path: PathBuf,
        num_vcpus: u8,
        initramfs_path: PathBuf,
        enable_network: bool,
        network_mac: String,
    ) -> Self {
        Self {
            mem_size_mb,
            kernel_path,
            num_vcpus,
            enable_network,
            network_mac,
            initramfs_path,
        }
    }
}

impl TryIntoVmmConfig<File> for VmmCliConfig {
    fn try_into_vmm_config(self) -> Result<VmmConfig<File>, Error> {
        let kernel = File::open(&self.kernel_path).map_err(Error::IO)?;
        let initramfs = File::open(&self.initramfs_path).map_err(Error::IO)?;

        Ok(VmmConfig {
            mem_size_mb: self.mem_size_mb,
            num_vcpus: self.num_vcpus,
            enable_network: self.enable_network,
            network_mac: self.network_mac.clone(),
            kernel,
            initramfs,
        })
    }
}
