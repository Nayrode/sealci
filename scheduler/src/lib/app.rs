use std::fmt::Display;
use std::sync::Arc;
use tokio::sync::Mutex;

// use logic::action_queue_logic::ActionsQueue;
use tonic::transport::Server;
use tracing::info;
use sealcid_traits::status::Status;
use crate::{
    interfaces::server::{agent_interface::AgentService, controller_interface::ControllerService},
    logic::agent_pool_logic::AgentPool,
    proto::{
        self,
        scheduler::{agent_server::AgentServer, controller_server::ControllerServer},
    },
};

//use proto::agent::agent_server::AgentServer;
//use proto::controller::controller_server::ControllerServer;

pub struct Config {
    pub addr: String,
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Config(addr: {})", self.addr)
    }
}

#[derive(Clone)]
pub struct App {
    pub addr: String,
}

impl sealcid_traits::App<Config> for App {
    type Error = Error;

    async fn run(&self) -> Result<(), Error> {
        let app_process = self.app_process.clone();
        let app_clone = self.clone();
        let mut process = app_process.write().await;
        *process = tokio::spawn(async move {
            let _ = app_clone.start();
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
        "SealCI".to_string()
    }
}


impl App {
    pub fn init(config: Config) -> Self {
        App { addr: config.addr }
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {

        // Initializes the Agent Pool and Action queue. They are lost when the Scheduler dies.
        let agent_pool = Arc::new(Mutex::new(AgentPool::new()));
        //let action_queue = Arc::new(Mutex::new(ActionsQueue::new()));

        // Pass the shared Agent Pool to Agent and Controller services.
        let agent = AgentService::new(agent_pool.clone());
        let controller = ControllerService::new(agent_pool.clone());

        let service = tonic_reflection::server::Builder::configure()
            .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
            .build()?;

        info!("Starting gRPC server at {}", self.addr);
        Server::builder()
            .add_service(service)
            .add_service(AgentServer::new(agent))
            .add_service(ControllerServer::new(controller))
            .serve(self.addr.parse()?)
            .await?;
        Ok(())
    }
}
