use clap::Parser;
use dumplet::{export_docker_image, create_initramfs, DumpletError};
use std::process::Command;
use std::path::Path;

/// Dumplet CLI: Export a Docker image and create an initramfs image.
#[derive(Parser, Debug)]
#[command(author, version, about = "Export a Docker image and create an initramfs image", long_about = None)]
struct Args {
    /// Docker image name (e.g. alpine:3.14)
    #[arg(help = "Docker image name (e.g. alpine:3.14)")]
    image: String,

    /// Path for the output directory containing tar.gz, extracted content, and .img file
    #[arg(help = "Path for the output directory")]
    output_dir: String,
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
    // Create output directory
    let output_path = Path::new(&args.output_dir);
    std::fs::create_dir_all(output_path)?;

    // Define paths within the output directory
    let rootfs_tar = output_path.join("rootfs.tar");
    let rootfs_tar_gz = output_path.join("rootfs.tar.gz");
    let extract_dir = output_path.join("rootfs-content");
    let initramfs_img = output_path.join("initramfs.img");

    // Export Docker image filesystem to a tar file
    println!("🔹 Exporting Docker image `{}` to {:?}", args.image, rootfs_tar);
    export_docker_image(&args.image, rootfs_tar.to_str().unwrap()).await?;

    // Compress the tar file to tar.gz
    println!("🔹 Compressing tar file to {:?}", rootfs_tar_gz);
    let status = Command::new("gzip")
        .args(["-c", rootfs_tar.to_str().unwrap()])
        .output()?;

    if !status.status.success() {
        return Err(DumpletError::IoError(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to compress tar file",
        )));
    }

    std::fs::write(&rootfs_tar_gz, status.stdout)?;

    // Extract tar to the content directory
    std::fs::create_dir_all(&extract_dir)?;
    println!("🔹 Extracting rootfs tar to {:?}", extract_dir);
    let status = Command::new("tar")
        .args(["xf", rootfs_tar.to_str().unwrap(), "-C", extract_dir.to_str().unwrap()])
        .status()?;
    if !status.success() {
        return Err(DumpletError::IoError(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to extract rootfs",
        )));
    }

    // Create the initramfs image
    println!("🔹 Creating initramfs image at {:?}", initramfs_img);
    create_initramfs(extract_dir.to_str().unwrap(), initramfs_img.to_str().unwrap())?;

    // Summary
    println!("✅ Build completed successfully!");
    println!("📁 Output directory: {}", args.output_dir);
    println!("   ├── rootfs.tar (original tar file)");
    println!("   ├── rootfs.tar.gz (compressed tar file)");
    println!("   ├── rootfs-content/ (extracted filesystem)");
    println!("   └── initramfs.img (final initramfs image)");

    Ok(())
}