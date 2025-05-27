use clap::Parser;
use dumplet::{export_docker_image, create_initramfs, DumpletError};
use std::process::Command;

/// Dumplet CLI: Export a Docker image and create an initramfs image.
#[derive(Parser, Debug)]
#[command(author, version, about = "Export a Docker image and create an initramfs image", long_about = None)]
struct Args {
    /// Docker image name (e.g. alpine:3.14)
    #[arg(help = "Docker image name (e.g. alpine:3.14)")]
    image: String,

    /// Path for the output initramfs image (e.g. /tmp/initramfs.img)
    #[arg(help = "Path for the output initramfs image")]
    initramfs_img: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if let Err(e) = run(&args).await {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

async fn run(args: &Args) -> Result<(), DumpletError> {
    // Export Docker image filesystem to a tar file
    let rootfs_tar = "/tmp/rootfs.tar";
    println!("🔹 Exporting Docker image `{}` to {}", args.image, rootfs_tar);
    export_docker_image(&args.image, rootfs_tar).await?;

    // Extract tar to a temporary directory
    let extract_dir = "/tmp/rootfs-extract";
    std::fs::create_dir_all(extract_dir)?;
    println!("🔹 Extracting rootfs tar to {}", extract_dir);
    let status = Command::new("tar")
        .args(["xf", rootfs_tar, "-C", extract_dir])
        .status()?;
    if !status.success() {
        return Err(DumpletError::IoError(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to extract rootfs",
        )));
    }

    // Create the initramfs image
    println!("🔹 Creating initramfs image at {}", args.initramfs_img);
    create_initramfs(extract_dir, &args.initramfs_img)?;

    Ok(())
}

