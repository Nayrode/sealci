use std::{fs::File, io::copy, path::PathBuf};

use crate::DumpletError;

pub struct TransferPath(PathBuf, PathBuf);

impl TransferPath {
    /// Returns the host path.
    pub fn host_path(&self) -> &PathBuf {
        &self.0
    }

    /// Returns the guest path.
    pub fn guest_path(&self) -> &PathBuf {
        &self.1
    }
}

impl TryFrom<String> for TransferPath {
    type Error = DumpletError;
    /// It must look like this:
    /// /path/on/the/host.txt:/path/on/the/guest.txt
    /// Converts a string into a TransferPath instance.
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split(':').collect();
        if parts.len() != 2 {
            return Err(DumpletError::InvalidFormat);
        }

        let host_path = PathBuf::from(parts[0]);
        let guest_path = PathBuf::from(parts[1]);
        Ok(TransferPath(host_path, guest_path))
    }
}

impl TryInto<TransferredFilePathIntoVm> for TransferPath {
    type Error = DumpletError;

    /// Converts a TransferPath instance into a TransferredFilePath instance.
    /// This involves checking if the paths are valid files.
    fn try_into(self) -> Result<TransferredFilePathIntoVm, Self::Error> {
        let host_path = self.0;
        let guest_path = self.1;

        if !host_path.is_file() || !guest_path.is_file() {
            return Err(DumpletError::InvalidPath);
        }

        Ok(TransferredFilePathIntoVm(host_path, guest_path))
    }
}

pub struct TransferredFilePathIntoVm(PathBuf, PathBuf);

impl TransferredFilePathIntoVm {
    pub fn new(rootfs_path: PathBuf, transfer: TransferPath) -> Result<Self, DumpletError> {
        let guest_path = transfer.guest_path();
        let vm_path = PathBuf::from(format!("{}{}", rootfs_path.display(), guest_path.display()));

        Ok(Self(transfer.host_path().to_owned(), vm_path))
    }
    /// Returns the host path.
    pub fn host_path(&self) -> &PathBuf {
        &self.0
    }

    /// Returns the guest path.
    pub fn guest_path(&self) -> &PathBuf {
        &self.1
    }
}

pub struct TransferredFile {
    host_file: File,
    guest_file: File,
}

impl TransferredFile {
    pub fn transfer(&mut self) -> Result<(), DumpletError> {
        // Here you would implement the logic to transfer the file from host_file to guest_file.
        // This is a placeholder implementation.
        println!(
            "Transferring file from {:?} to {:?}",
            self.host_file, self.guest_file
        );
        copy(&mut self.host_file, &mut self.guest_file)?;
        Ok(())
    }
}

impl TryFrom<TransferredFilePathIntoVm> for TransferredFile {
    type Error = DumpletError;
    /// Converts a transfer path string to a TransferredFile instance.
    /// And it must reference files not folders.
    fn try_from(transfer: TransferredFilePathIntoVm) -> Result<Self, Self::Error> {
        let host_path = transfer.host_path();
        println!("Host path: {:?}", host_path);
        let guest_path = transfer.guest_path();
        println!("Guest path: {:?}", guest_path);
        let host_file = File::open(host_path).map_err(DumpletError::IoError)?;
        if let Some(parent) = guest_path.parent() {
            std::fs::create_dir_all(parent).map_err(DumpletError::IoError)?;
        }
        let guest_file = File::create(guest_path).map_err(DumpletError::IoError)?;

        let mut tfr_file = TransferredFile {
            host_file,
            guest_file,
        };
        tfr_file.transfer()?;
        Ok(tfr_file)
    }
}
