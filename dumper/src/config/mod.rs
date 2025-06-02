use std::fs::File;
use std::io::{Read, Seek};

use macro_vmm::TryIntoVmm;
use vm_memory::ReadVolatile;

use crate::common::error::Error;
use crate::vmm::VMM;
pub mod cli;

pub struct VmmConfig<T: Read + ReadVolatile + Seek> {
    pub mem_size_mb: u32,
    pub num_vcpus: u8,
    pub kernel: T,
    pub initramfs: File,
    pub enable_network: bool,
    pub network_mac: String,
    pub tap_interface_name: String,
}

#[allow(dead_code)]
impl<T: Read + ReadVolatile + Seek> VmmConfig<T> {
    async fn try_into_vmm(self) -> Result<VMM, Error> {
        VMM::new(self).await
    }
}

pub trait TryIntoVmmConfig<T: Read + ReadVolatile + Seek> {
    fn try_into_vmm_config(self) -> Result<VmmConfig<T>, Error>;
}

pub trait TryIntoVmm {
    async fn try_into_vmm(self) -> Result<VMM, Error>;
}
