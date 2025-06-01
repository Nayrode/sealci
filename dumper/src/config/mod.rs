use std::fs::File;

pub mod vmm;

pub struct VmmConfig {
    pub mem_size_mb: u32,
    pub num_vcpus: u8,
    pub kernel: File,
    pub initramfs: File,
}
