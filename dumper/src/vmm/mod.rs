use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use std::thread;

use kvm_bindings::kvm_userspace_memory_region;
use kvm_ioctls::{Kvm, VmFd};
use linux_loader::loader::KernelLoaderResult;
use vm_memory::{Address, GuestAddress, GuestMemory, GuestMemoryMmap, GuestMemoryRegion};

use crate::{common::error::Error, devices::{epoll::EpollContext, serial::DumperSerial}};
use crate::config::vmm::VmmConfig;
use crate::cpu::{self, mptable, Vcpu};
use crate::kernel;

pub struct VMM {
    vm_fd: VmFd,
    kvm: Kvm,
    guest_memory: GuestMemoryMmap,
    vcpus: Vec<Vcpu>,
    serial: Arc<Mutex<DumperSerial>>,
    epoll: EpollContext,
}

impl VMM {
    pub fn new() -> Result<Self, Error> {
        let kvm = Kvm::new().map_err(Error::KvmIoctl)?;
        let vm_fd = kvm.create_vm().map_err(Error::KvmIoctl)?;
        let serial = Arc::new(Mutex::new(
            DumperSerial::new().map_err(Error::SerialCreation)?,
        ));

        let guest_memory = GuestMemoryMmap::default();

        let epoll = EpollContext::new().map_err(Error::EpollError)?;
        epoll.add_stdin().map_err(Error::EpollError)?;

        let vmm = VMM {
            vm_fd,
            kvm,
            guest_memory,
            serial,
            epoll,
            vcpus: vec![],
        };

        Ok(vmm)
    }

    pub fn configure_memory(&mut self, mem_size_mb: u32) -> Result<(), Error> {
        // Convert memory size from MBytes to bytes.
        let mem_size = ((mem_size_mb as u64) << 20) as usize;

        // Create one single memory region, from zero to mem_size.
        let mem_regions = vec![(GuestAddress(0), mem_size)];

        // Allocate the guest memory from the memory region.
        let guest_memory = GuestMemoryMmap::from_ranges(&mem_regions).map_err(Error::Memory)?;

        // For each memory region in guest_memory:
        // 1. Create a KVM memory region mapping the memory region guest lphysical address to the host virtual address.
        // 2. Register the KVM memory region with KVM. EPTs are created then.
        for (index, region) in guest_memory.iter().enumerate() {
            let kvm_memory_region = kvm_userspace_memory_region {
                slot: index as u32,
                guest_phys_addr: region.start_addr().raw_value(),
                memory_size: region.len() as u64,
                // It's safe to unwrap because the guest address is valid.
                userspace_addr: guest_memory.get_host_address(region.start_addr()).unwrap() as u64,
                flags: 0,
            };

            // Register the KVM memory region with KVM.
            unsafe { self.vm_fd.set_user_memory_region(kvm_memory_region) }
                .map_err(Error::KvmIoctl)?;
        }

        self.guest_memory = guest_memory;

        Ok(())
    }

    pub fn configure_vcpus(
        &mut self,
        num_vcpus: u8,
        kernel_load: KernelLoaderResult,
    ) -> Result<(), Error> {
        mptable::setup_mptable(&self.guest_memory, num_vcpus)
            .map_err(|e| Error::Vcpu(cpu::Error::Mptable(e)))?;

        for index in 0..num_vcpus {
            let vcpu = Vcpu::new(&self.vm_fd, index.into()).map_err(Error::Vcpu)?;

            // Configure MSRs (model specific registers).
            vcpu.configure_msrs().map_err(Error::Vcpu)?;
            // Configure regs, sregs and fpu.
            vcpu.configure_regs(kernel_load.kernel_load)
                .map_err(Error::Vcpu)?;
            vcpu.configure_sregs(&self.guest_memory)
                .map_err(Error::Vcpu)?;
            vcpu.configure_fpu().map_err(Error::Vcpu)?;

            // Configure LAPICs.
            vcpu.configure_lapic().map_err(Error::Vcpu)?;
            self.vcpus.push(vcpu);
        }

        Ok(())
    }

    // Run all virtual CPUs.
    pub fn run(&mut self) -> Result<(), Error> {
        for mut vcpu in self.vcpus.drain(..) {
            println!("Starting vCPU {:?}", vcpu.index);
            let _ = thread::Builder::new().spawn(move || loop {
                vcpu.run();
            });
        }

        Ok(())
    }

    pub fn configure(
        &mut self,
        config: VmmConfig
    ) -> Result<(), Error> {
        self.configure_memory(config.mem_size_mb)?;
        let kernel_load = kernel::kernel_setup(&self.guest_memory, PathBuf::from(config.kernel_path))?;
        self.configure_io()?;
        self.configure_vcpus(config.num_vcpus, kernel_load)?;
        Ok(())
    }

    pub fn configure_io(&mut self) -> Result<(), Error> {
        // First, create the irqchip.
        // On `x86_64`, this _must_ be created _before_ the vCPUs.
        // It sets up the virtual IOAPIC, virtual PIC, and sets up the future vCPUs for local APIC.
        // When in doubt, look in the kernel for `KVM_CREATE_IRQCHIP`.
        // https://elixir.bootlin.com/linux/latest/source/arch/x86/kvm/x86.c
        self.vm_fd.create_irq_chip().map_err(Error::KvmIoctl)?;
        let serial = &self
            .serial
            .lock()
            .unwrap()
            .eventfd()
            .map_err(Error::IrqRegister)?;

        self.vm_fd
            .register_irqfd(serial, 4)
            .map_err(Error::KvmIoctl)?;

        Ok(())
    }
}
