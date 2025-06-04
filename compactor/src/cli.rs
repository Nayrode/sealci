use clap::{command, Parser};

/// Dumplet CLI: Export a Docker image and create an initramfs image.
#[derive(Parser, Debug)]
#[command(author, version, about = "Export a Docker image and create an initramfs image", long_about = None)]
pub struct Cli {
    #[arg(help = "Docker image name (e.g. alpine:3.14)")]
    pub image: String,
    
    #[arg(
        long,
        short,
        default_value = "tap0",
        help = "Name of the tap interface to use for networking on the host"
    )]
    pub tap_interface_name: String,
    
    #[arg(short = 'i', long = "it", help = "Enable interactive mode", action = clap::ArgAction::SetTrue)]
    pub interactive: bool,

    #[arg(long, help = "Environment variables to pass (e.g. --env KEY=VALUE)", num_args = 0.., value_delimiter = ',')]
    pub env: Vec<String>,

    #[arg(long, help = "List of files to transfer in the format /path/on/host:/path/on/guest", num_args = 0.., value_delimiter = ',')]
    pub transfer_files: Vec<String>,
    
    #[arg(long, default_value = "2048", help = "Memory size in megabytes")]
    pub mem_size_mb: u32,


    #[arg(long, default_value = "4", help = "Number of virtual CPUs")]
    pub num_vcpus: u8,
}
