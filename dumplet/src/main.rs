use clap::Parser;
use dumplet::{generate_initramfs_bundle, DumpletError};

/// Dumplet CLI: Export a Docker image and create an initramfs image.
#[derive(Parser, Debug)]
#[command(author, version, about = "Export a Docker image and create an initramfs image", long_about = None)]
struct Args {
    #[arg(help = "Docker image name (e.g. alpine:3.14)")]
    image: String,

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
    let bundle = generate_initramfs_bundle(&args.image, &args.output_dir).await?;

    println!("🔹 Build completed successfully!");
    println!("🔹 Output directory: {}", args.output_dir);
    println!("   ├── {}", bundle.rootfs_tar.display());
    println!("   ├── {}", bundle.rootfs_tar_gz.display());
    println!("   ├── {}", bundle.extract_dir.display());
    println!("   └── {}", bundle.initramfs_img.display());

    Ok(())
}
