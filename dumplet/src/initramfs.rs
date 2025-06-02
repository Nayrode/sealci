use std::fs::{self, File};
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::{Command, Stdio};

use crate::errors::DumpletError;

pub fn create_initramfs(rootfs_path: &Path, output_img: &Path) -> Result<(), DumpletError> {
    let init_path = rootfs_path.join("init");

    let init_script = r#"#!/bin/sh
        mount -t devtmpfs dev /dev
        mount -t proc proc /proc
        mount -t sysfs sysfs /sys
        ip link set up dev lo

        exec /sbin/getty -n -l /bin/sh 115200 /dev/console
        poweroff -f
    "#;

    let mut file = File::create(&init_path)?;
    file.write_all(init_script.as_bytes())?;

    let mut perms = fs::metadata(&init_path)?.permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&init_path, perms)?;

    let find = Command::new("find")
        .arg(".")
        .arg("-print0")
        .current_dir(rootfs_path)
        .stdout(Stdio::piped())
        .spawn()?;

    let cpio = Command::new("cpio")
        .args(["--null", "--create", "--verbose", "--owner", "root:root", "--format=newc"])
        .stdin(find.stdout.unwrap())
        .stdout(Stdio::piped())
        .current_dir(rootfs_path)
        .spawn()?;

    let mut xz = Command::new("xz")
        .args(["-9", "--format=lzma"])
        .stdin(cpio.stdout.unwrap())
        .stdout(File::create(output_img)?)
        .spawn()?;

    let status = xz.wait()?;
    if !status.success() {
        return Err(DumpletError::IoError(io::Error::new(io::ErrorKind::Other, "Failed to create initramfs image")));
    }

    println!("🔹 Initramfs image created: {:?}", output_img);
    Ok(())
}
