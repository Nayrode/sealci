use std::fmt::Pointer;
use std::net::Ipv4Addr;
use std::ops::DerefMut;
use std::{
    io,
    os::fd::{AsRawFd, RawFd},
    path::PathBuf,
    sync::{Arc, Mutex},
};

mod irq_allocator;
use event_manager::{EventManager, MutEventSubscriber};
use kvm_bindings::{kvm_userspace_memory_region, KVM_MAX_CPUID_ENTRIES};
use kvm_ioctls::{Kvm, VmFd};
use linux_loader::loader::{Cmdline, KernelLoaderResult};
use std::thread;
use vm_allocator::{AddressAllocator, AllocPolicy};
use vm_device::bus::{MmioAddress, MmioRange};
use vm_device::device_manager::IoManager;
use vm_memory::{Address, GuestAddress, GuestMemory, GuestMemoryMmap, GuestMemoryRegion};
use vmm_sys_util::terminal::Terminal;

use crate::cpu::{self, mptable, Vcpu};
use crate::devices::virtio;
use crate::devices::virtio::net::Net;
use crate::devices::virtio::{Env, MmioConfig};
use crate::kernel;
use crate::vmm::irq_allocator::IrqAllocator;
use crate::{
    common::error::Error,
    cpu::cpuid,
    devices::{epoll::EpollContext, serial::DumperSerial},
};
use crate::{config::vmm::VmmConfig, devices::epoll::EPOLL_EVENTS_LEN};

#[cfg(target_arch = "x86_64")]
pub(crate) const MMIO_GAP_END: u64 = 1 << 34;
/// Size of the MMIO gap.
#[cfg(target_arch = "x86_64")]
pub(crate) const MMIO_GAP_SIZE: u64 = 768 << 20;
/// The start of the MMIO gap (memory area reserved for MMIO devices).
#[cfg(target_arch = "x86_64")]
pub(crate) const MMIO_GAP_START: u64 = MMIO_GAP_END - MMIO_GAP_SIZE;
pub const DEFAULT_ADDRESS_ALIGNEMNT: u64 = 4;
/// Default allocation policy for address allocator.
pub const DEFAULT_ALLOC_POLICY: AllocPolicy = AllocPolicy::FirstMatch;
/// IRQ line 4 is typically used for serial port 1.
// See more IRQ assignments & info: https://tldp.org/HOWTO/Serial-HOWTO-8.html
const SERIAL_IRQ: u32 = 4;
/// Last usable IRQ ID for virtio device interrupts on x86_64.
const IRQ_MAX: u8 = 23;

type EventMgr = EventManager<Arc<Mutex<dyn MutEventSubscriber + Send>>>;

pub struct VMM {
    vm_fd: Arc<VmFd>,
    kvm: Kvm,
    guest_memory: GuestMemoryMmap,
    vcpus: Vec<Vcpu>,
    net_devices: Vec<Arc<Mutex<Net<Arc<GuestMemoryMmap>>>>>,
    address_allocator: Option<AddressAllocator>,
    irq_allocator: IrqAllocator,
    event_mgr: Arc<Mutex<EventMgr>>,
    device_mgr: Arc<Mutex<IoManager>>,
    cmdline: Cmdline,
    serial: Arc<Mutex<DumperSerial>>,
    epoll: EpollContext,
}

impl VMM {
    pub fn new() -> Result<Self, Error> {
        let kvm = Kvm::new().map_err(Error::KvmIoctl)?;
        let vm_fd = Arc::new(kvm.create_vm().map_err(Error::KvmIoctl)?);
        let mut cmdline = Cmdline::new(16384).map_err(Error::Cmdline)?;
        cmdline
            .insert_str(crate::kernel::CMDLINE)
            .map_err(Error::Cmdline)?;
        let device_mgr = Arc::new(Mutex::new(IoManager::new()));
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
            address_allocator: None,
            net_devices: Vec::new(),
            irq_allocator: IrqAllocator::new(SERIAL_IRQ, IRQ_MAX.into()).unwrap(),
            event_mgr: Arc::new(Mutex::new(EventManager::new().unwrap())),
            device_mgr,
            cmdline,
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

    fn configure_allocators(&mut self, mem_size_mb: u32) -> Result<(), Error> {
        // Convert memory size from MBytes to bytes.
        let mem_size = (mem_size_mb as u64) << 20;

        // Setup address allocator.
        let start_addr = MMIO_GAP_START;
        let address_allocator = AddressAllocator::new(start_addr, mem_size).unwrap();

        self.address_allocator = Some(address_allocator);

        Ok(())
    }

    pub fn configure_vcpus(
        &mut self,
        num_vcpus: u8,
        kernel_load: KernelLoaderResult,
    ) -> Result<(), Error> {
        mptable::setup_mptable(&self.guest_memory, num_vcpus)
            .map_err(|e| Error::Vcpu(cpu::Error::Mptable(e)))?;

        let base_cpuid = self
            .kvm
            .get_supported_cpuid(KVM_MAX_CPUID_ENTRIES)
            .map_err(Error::KvmIoctl)?;

        for index in 0..num_vcpus {
            let vcpu = Vcpu::new(
                &self.vm_fd,
                index.into(),
                self.serial.clone(),
                self.device_mgr.clone(),
            )
            .map_err(Error::Vcpu)?;
            // Set CPUID.
            let mut vcpu_cpuid = base_cpuid.clone();
            cpuid::filter_cpuid(
                &self.kvm,
                index as usize,
                num_vcpus as usize,
                &mut vcpu_cpuid,
            );
            vcpu.configure_cpuid(&vcpu_cpuid).map_err(Error::Vcpu)?;

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

    pub async fn configure_net_device(&mut self) -> Result<(), Error> {
        let mem = Arc::new(self.guest_memory.clone());
        let range = &self
            .address_allocator
            .clone()
            .ok_or(Error::NoAllocatorFound)?
            .to_owned()
            .allocate(0x1000, DEFAULT_ADDRESS_ALIGNEMNT, DEFAULT_ALLOC_POLICY)
            .map_err(Error::AllocationError)?;

        let mmio_range =
            MmioRange::new(MmioAddress(range.start()), range.len()).map_err(Error::Mmio)?;
        let irq = self.irq_allocator.next_irq()?;
        let mmio_cfg = MmioConfig {
            range: mmio_range,
            gsi: irq,
        };

        let mut device_guard = self.device_mgr.lock().unwrap();
        let mut event_guard = self.event_mgr.lock().unwrap();

        let mut env = Env {
            mem: Arc::new(self.guest_memory.clone()),
            event_mgr: event_guard.deref_mut(),
            mmio_mgr: device_guard.deref_mut(),
            mmio_cfg,
            vm_fd: self.vm_fd.clone(),
            kernel_cmdline: &mut self.cmdline,
        };

        let net_args = virtio::net::NetArgs {
            tap_name: "tap0".to_string(),
        };

        let net = Net::new(
            mem,
            &mut env,
            &net_args,
            Ipv4Addr::new(192, 168, 1, 1),
            Ipv4Addr::new(255, 255, 255, 0),
            Ipv4Addr::new(192, 168, 1, 2),
        )
        .await
        .map_err(Error::NetError)?;

        self.net_devices.push(net);

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
        let stdin = io::stdin();
        let stdin_lock = stdin.lock();
        stdin_lock
            .set_raw_mode()
            .map_err(Error::TerminalConfigure)?;
        let mut events = vec![epoll::Event::new(epoll::Events::empty(), 0); EPOLL_EVENTS_LEN];
        let epoll_fd = self.epoll.as_raw_fd();

        let event_mgr = self.event_mgr.clone();
        let _ = thread::Builder::new().spawn(move || loop {
            match event_mgr.lock().unwrap().run() {
                Ok(_) => (),
                Err(e) => eprintln!("Failed to handle events: {:?}", e),
            }
        });

        // Let's start the STDIN polling thread.
        loop {
            let num_events =
                epoll::wait(epoll_fd, -1, &mut events[..]).map_err(Error::EpollError)?;

            for event in events.iter().take(num_events) {
                let event_data = event.data as RawFd;

                if let libc::STDIN_FILENO = event_data {
                    let mut out = [0u8; 64];

                    let count = stdin_lock.read_raw(&mut out).map_err(Error::StdinRead)?;

                    self.serial
                        .lock()
                        .unwrap()
                        .serial
                        .enqueue_raw_bytes(&out[..count])
                        .map_err(Error::StdinWrite)?;
                }
            }
        }
    }

    pub async fn configure(&mut self, config: VmmConfig) -> Result<(), Error> {
        self.configure_memory(config.mem_size_mb)?;
        self.configure_allocators(config.mem_size_mb)?;
        let kernel_load = kernel::kernel_setup(
            &self.guest_memory,
            PathBuf::from(config.kernel_path),
            &self.cmdline,
            &config.initramfs_path,
        )?;
        self.configure_io()?;
        self.configure_net_device().await?;
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
