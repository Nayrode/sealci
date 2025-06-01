use std::fs::File;

use macro_vmm::TryIntoVmm;

use crate::common::error::Error;
use crate::vmm::VMM;
pub mod cli;

#[derive(TryIntoVmm)]
pub struct VmmConfig {
    pub mem_size_mb: u32,
    pub num_vcpus: u8,
    pub kernel: File,
    pub initramfs: File,
}

impl TryIntoVmmConfig for VmmConfig {
    fn try_into(self) -> Result<VmmConfig, Error> {
        Ok(self)
    }
}

pub trait TryIntoVmmConfig {
    fn try_into(self) -> Result<VmmConfig, Error>;
}

pub trait TryIntoVmm {
    fn try_into(self) -> Result<VMM, Error>;
}


