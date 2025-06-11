use std::io::Cursor;
use std::sync::Arc;
use tokio::sync::{watch, MutexGuard};
use sealcid_traits::proto::ServiceStatus as Status;
use tonic::transport::Server;
use tracing::info;
use dumper::config::VmmConfig;
use super::Error;

pub use crate::{config::Config, Compactor};
use crate::kernel::VMLINUX;

impl sealcid_traits::App<Config> for Compactor {
    type Error = Error;

    async fn run(&self) -> Result<(), Error> {
        let app_process = self.app_process.clone();
        let app_clone = self.clone();
        let (send, mut recv) = watch::channel::<()>(());
        
        let handle = recv.clone();
        let process = app_process.write().await;
        let mut app = self.clone();
        let task = tokio::spawn(async move {
            // Run the blocking code inside spawn_blocking
            println!("Starting Compactor service...");
            let handlers = app.vmm.lock().await.run(false);
            println!("{:?}", handlers);
        });
        tokio::spawn(async move {
            tokio::select! {
            _ = task => {}
            _ = recv.changed() => {
                info!("Received shutdown signal, stopping the app...");
            }
        };});
        
        Ok(())
    }

    async fn configure(config: Config) -> Result<Self, Error> {
        Self::new(config).await
    }

    async fn stop(&self) -> Result<(), Error> {
        let app_process = self.app_process.clone();
        if let Some(vcpu_handle) = self.vcpu_handle.clone().write().await.take() {
            vcpu_handle.send(()).expect("TODO: panic message");
        }
        if let Some(handle) = self.handle.clone().write().await.take() {
            handle.send(()).expect("TODO: panic message");
        }
        let process = app_process.read().await;
        process.abort();
        Ok(())
    }

    async fn configuration(&self) -> Result<impl std::fmt::Display, Error> {
        Ok(self.config.clone())
    }

    async fn status(&self) -> Status {
        let app_process = self.app_process.read().await;
        if app_process.is_finished() {
            // Try to get the result without blocking
            Status::Stopped
        } else {
            Status::Running
        }
    }

    fn name(&self) -> String {
        "Release agent".to_string()
    }
}
