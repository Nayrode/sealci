use std::{fs::File, path::PathBuf};

use clap::{self, arg, command, Parser};

use crate::common::error::Error;

use super::VmmConfig;

#[derive(Parser)]
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
    pub initramfs_path: Option<PathBuf>,
}

impl VmmConfig {
    pub fn new(
        mem_size_mb: u32,
        kernel_path: String,
        num_vcpus: u8,
        initramfs_path: Option<PathBuf>,
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

impl TryInto<VmmConfig> for VmmCliConfig {
    type Error = Error;

    fn try_into(self) -> Result<VmmConfig, Self::Error> {
        Ok(VmmConfig {
            mem_size_mb: self.mem_size_mb,
            num_vcpus: self.num_vcpus,
            kernel: File::open(self.kernel_path).map_err(Error::IO)?,
            initramfs: File::open(self.initramfs_path).map_err(Error::IO)?,
        })
    }
}
