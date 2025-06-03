mod docker;
pub mod errors;
mod initramfs;
mod tar_utils;
pub mod transferred_file;

use std::fs::{self, File};
use std::path::{Path, PathBuf};
use tempfile::tempdir;

use docker::export_docker_image;
pub use errors::DumpletError;
use initramfs::create_initramfs;
use tar_utils::{compress_tar, extract_tar};

#[derive(Debug)]
pub struct DumpletBundle {
    pub rootfs_tar: PathBuf,
    pub rootfs_tar_gz: PathBuf,
    pub extract_dir: PathBuf,
    pub initramfs_img: PathBuf,
}

pub async fn generate_initramfs_bundle(
    image: &str,
    output_dir: &str,
    env: Option<Vec<&str>>,
    transfer_files: Vec<String>,
) -> Result<DumpletBundle, DumpletError> {
    let output_path = Path::new(output_dir);
    fs::create_dir_all(output_path)?;

    let rootfs_tar = output_path.join("rootfs.tar");
    let rootfs_tar_gz = output_path.join("rootfs.tar.gz");
    let extract_dir = output_path.join("rootfs-content");
    let initramfs_img = output_path.join("initramfs.img");

    let (container_cmd, working_dir) = export_docker_image(image, &rootfs_tar).await?;

    compress_tar(&rootfs_tar, &rootfs_tar_gz)?;
    extract_tar(&rootfs_tar, &extract_dir)?;
    create_initramfs(
        &extract_dir,
        &initramfs_img,
        env,
        container_cmd,
        working_dir,
        transfer_files,
    )?;

    Ok(DumpletBundle {
        rootfs_tar,
        rootfs_tar_gz,
        extract_dir,
        initramfs_img,
    })
}

pub async fn generate_initramfs_image(
    image: &str,
    env: Option<Vec<&str>>,
    transfer_files: Vec<String>,
) -> Result<File, DumpletError> {
    let temp_dir = tempdir()?;
    let temp_path = temp_dir.path();

    let bundle =
        generate_initramfs_bundle(image, temp_path.to_str().unwrap(), env, transfer_files).await?;
    let file = File::open(&bundle.initramfs_img)?;

    println!(
        "🔹 Generated initramfs.img file: {:?}",
        bundle.initramfs_img
    );
    Ok(file)
}
