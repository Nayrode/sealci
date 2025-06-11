use std::io::Cursor;
use std::sync::Arc;
use tokio::sync::oneshot::Sender;
use tokio::sync::{oneshot, watch, Mutex, RwLock};
use dumper::config::VmmConfig;
use dumper::vmm::VMM;

use crate::{config::Config, error::Error, kernel::VMLINUX};

pub mod config;
pub mod error;
pub mod kernel;
pub mod app;

#[derive(Clone)]
pub struct Compactor {
    config: Config,
    vcpu_handle: Arc<RwLock<Option<watch::Sender<()>>>>,
    handle: Arc<RwLock<Option<oneshot::Sender<()>>>>,
    app_process: Arc<RwLock<tokio::task::JoinHandle<Result<(), Error>>>>,
    vmm: Arc<Mutex<VMM>>,
}

impl Compactor {
    pub async fn new(conf: Config) -> Result<Self, Error> {
        let kernel = Cursor::new(VMLINUX);
        let config = conf.clone();
        let mut envs = Vec::new();
        for env in &config.env {
            envs.push(env.as_str());
        }
        let initramfs =
            dumplet::generate_initramfs_image(&config.image, Some(envs), config.transfer_files, config.dns)
                .await
                .map_err(Error::DumpletError).unwrap();
        let vmm_config = dumper::config::VmmConfig {
            mem_size_mb: config.mem_size_mb,
            num_vcpus: config.num_vcpus,
            kernel: kernel,
            initramfs: initramfs,
            enable_network: true,
            network_mac: "".to_string(),
            tap_interface_name: config.tap_interface_name.clone(),
        };
        let vmm = tokio::task::spawn_blocking(|| {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("Failed to create tokio runtime");;
            rt.block_on(vmm_config
                .try_into_vmm()).unwrap()
        });
        Ok(Self {
            config: conf,
            handle: Arc::new(RwLock::new(None)),
            vcpu_handle: Arc::new(RwLock::new(None)),
            app_process: Arc::new(RwLock::new(tokio::task::spawn(async { Ok(()) }))),
            vmm: Arc::new(Mutex::new(vmm.await.unwrap())),
        })
    }

    pub async fn run(&mut self) {
        let (send, recv) = watch::channel::<()>(());
        *self.vcpu_handle.write().await = Some(send);
        
    }
}
