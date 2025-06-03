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

    #[arg(long, help = "Environment variables to pass (e.g. --env KEY=VALUE)", num_args = 0.., value_delimiter = ',')]
    env: Vec<String>,

    #[arg(long, help = "List of files to transfer in the format /path/on/host:/path/on/guest", num_args = 0.., value_delimiter = ',')]
    transfer_files: Vec<String>,
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
    let env_vars: Option<Vec<&str>> = if !args.env.is_empty() {
        Some(args.env.iter().map(|s| s.as_str()).collect())
    } else {
        None
    };

    let bundle = generate_initramfs_bundle(
        &args.image,
        &args.output_dir,
        env_vars,
        args.transfer_files.clone(),
    )
    .await?;

    println!("🔹 Build completed successfully!");
    println!("🔹 Output directory: {}", args.output_dir);
    println!("   ├── {}", bundle.rootfs_tar.display());
    println!("   ├── {}", bundle.rootfs_tar_gz.display());
    println!("   ├── {}", bundle.extract_dir.display());
    println!("   └── {}", bundle.initramfs_img.display());

    Ok(())
}
