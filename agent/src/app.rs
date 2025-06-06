use std::{
    net::{AddrParseError, SocketAddr},
    sync::Arc,
};
use std::time::Duration;
use bollard::Docker;
use sealcid_traits::status::Status;
use tokio::{sync::RwLock, task};
use tokio::time::sleep;
use tonic::transport::Server;
use tracing::{error, info};

use crate::{
    brokers::state_broker::StateBroker,
    config::Config,
    models::error::Error,
    proto::action_service_server::ActionServiceServer,
    server::ActionsLauncher,
    services::{
        action_service::ActionService, health_service::HealthService,
        scheduler_service::SchedulerService,
    },
};
use crate::models::error::Error::ConnectionError;

#[derive(Clone)]
pub struct App {
    config: Config,
    health_service: HealthService,
    action_service_grpc: ActionServiceServer<ActionsLauncher>,
    app_process: Arc<RwLock<tokio::task::JoinHandle<Result<(), Error>>>>,
}

impl sealcid_traits::App<Config> for App {
    type Error = Error;

    async fn run(&self) -> Result<(), Error> {
        let app_process = self.app_process.clone();
        let mut app_clone = self.clone();
        let mut process = app_process.write().await;
        *process = tokio::spawn(async move {
            let _ = app_clone.start().await?;
            Ok(())
        });
        Ok(())
    }

    async fn configure(config: Config) -> Result<Self, Error> {
        Self::init(config).await
    }

    async fn stop(&self) -> Result<(), Error> {
        let app_process = self.app_process.clone();
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
        "Agent".to_string()
    }
}

impl App {
    pub async fn init(config: Config) -> Result<Self, Error> {
        let health_service = HealthService::new();

        let docker = Arc::new(Docker::connect_with_socket_defaults().unwrap());
        docker.ping().await.map_err(Error::DockerConnectionError)?;

        let state_broker = Arc::new(StateBroker::new());
        let action_service = ActionService::new(docker, state_broker.clone());
        let actions = ActionsLauncher { action_service };
        let action_service_grpc = ActionServiceServer::new(actions);
        
        Ok(Self {
            action_service_grpc,
            config,
            health_service,
            app_process: Arc::new(RwLock::new(tokio::spawn(async { Ok(()) }))),
        })
    }

    pub async fn start(&mut self) -> Result<(), Error> {// Exponential backoff configuration
        let mut retry_delay = Duration::from_secs(2);
        const MAX_RETRY_DELAY: u64 = 64;
        let mut retry_count = 0;
        let mut scheduler_service = loop {
            match SchedulerService::init(
                self.config.shost.clone(),
                self.config.ahost.clone(),
                self.config.port.clone(),
                self.health_service.clone(),
            )
                .await {
                Ok(client) => break client,
                Err(e) => {
                    if retry_count >= 10 {
                        return Err(e);
                    }
                    error!(
                        "Failed to connect to scheduler: {}, retrying in {:?} seconds...",
                        e, retry_delay
                    );
                    sleep(retry_delay).await;
                    retry_delay *= 2;
                    if retry_delay > Duration::from_secs(MAX_RETRY_DELAY) {
                        retry_delay = Duration::from_secs(MAX_RETRY_DELAY);
                    }
                    retry_count += 1;
                }
            }
        };
        scheduler_service.register().await?;
        let addr: SocketAddr = format!("0.0.0.0:{}", self.config.port)
            .parse()
            .map_err(|e: AddrParseError| Error::Error(e.to_string()))?;
        info!("Starting server on {}", addr);
        let server = Server::builder()
            .add_service(self.action_service_grpc.clone())
            .serve(addr);
        let health_report = task::spawn(async move {
            let _ = scheduler_service.report_health().await;
        });
        tokio::select! {
            serve_res = server => {
                serve_res
            .map_err(Error::ServeError)?;
            }
            health_report = health_report => {
                let _ = health_report;
            }
        };

        Ok(())
    }
}
