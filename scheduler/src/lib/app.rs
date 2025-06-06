use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::transport::Server;
use tracing::info;

use crate::{
    errors::Error,
    interfaces::server::{agent_interface::AgentService, controller_interface::ControllerService},
    logic::agent_pool_logic::AgentPool,
    proto::{
        self,
        scheduler::{agent_server::AgentServer, controller_server::ControllerServer},
    },
};

pub struct Config {
    pub addr: String,
}

pub struct App {
    pub addr: String,
}

impl App {
    pub fn new(config: Config) -> Self {
        App { addr: config.addr }
    }

    pub async fn run(&self) -> Result<(), Error> {
        // Initializes the Agent Pool and Action queue. They are lost when the Scheduler dies.
        let agent_pool = Arc::new(Mutex::new(AgentPool::new()));

        // Pass the shared Agent Pool to Agent and Controller services.
        let agent = AgentService::new(agent_pool.clone());
        let controller = ControllerService::new(agent_pool.clone());

        let service = tonic_reflection::server::Builder::configure()
            .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
            .build_v1()
            .map_err(|e| Error::GrpcSetupError(tonic::Status::internal(e.to_string())))?;

        info!("Starting gRPC server at {}", self.addr);
        Server::builder()
            .add_service(service)
            .add_service(AgentServer::new(agent))
            .add_service(ControllerServer::new(controller))
            .serve(self.addr.parse().map_err(|e| Error::AddrParseError(e))?)
            .await
            .map_err(|e| Error::GrpcServerError(tonic::Status::internal(e.to_string())))?;
        Ok(())
    }
}
