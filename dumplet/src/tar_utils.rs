use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;

use crate::errors::DumpletError;

pub fn compress_tar(tar_path: &Path, tar_gz_path: &Path) -> Result<(), DumpletError> {
    println!("🔹 Compressing tar file to {:?}", tar_gz_path);

    let output = Command::new("gzip")
        .args(["-c", tar_path.to_str().unwrap()])
        .output()?;

    if !output.status.success() {
        return Err(DumpletError::IoError(io::Error::new(
            io::ErrorKind::Other,
            "Failed to compress tar file",
        )));
    }

    fs::write(tar_gz_path, output.stdout)?;
    Ok(())
}

pub fn extract_tar(tar_path: &Path, extract_dir: &Path) -> Result<(), DumpletError> {
    fs::create_dir_all(extract_dir)?;
    println!("🔹 Extracting rootfs to {:?}", extract_dir);

    let status = Command::new("tar")
        .args(["xf", tar_path.to_str().unwrap(), "-C", extract_dir.to_str().unwrap()])
        .status()?;

    if !status.success() {
        return Err(DumpletError::IoError(io::Error::new(
            io::ErrorKind::Other,
            "Failed to extract rootfs",
        )));
    }
    Ok(())
}
