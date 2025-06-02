
use clap::{self, arg, command, Parser};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct VmmConfig {
    #[arg(long, default_value = "512", help = "Memory size in megabytes")]
    pub mem_size_mb: u32,

    #[arg(long, help = "Path to the kernel image")]
    pub kernel_path: String,

    #[arg(long, default_value = "1", help = "Number of virtual CPUs")]
    pub num_vcpus: u8,

    #[arg(long, default_value = "true", help = "Enable virtio network device")]
    pub enable_network: bool,

    #[arg(long, default_value = "52:54:00:12:34:56", help = "MAC address for the virtio network device")]
    pub network_mac: String,
}


impl VmmConfig {
    pub fn new(mem_size_mb: u32, kernel_path: String, num_vcpus: u8, enable_network: bool, network_mac: String) -> Self {
        Self {
            mem_size_mb,
            kernel_path,
            num_vcpus,
            enable_network,
            network_mac,
        }
    }
}
