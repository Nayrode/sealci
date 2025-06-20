use crate::core::ReleaseAgentError;
use crate::grpc::release_agent_grpc::release_agent_server::ReleaseAgent;
use crate::grpc::release_agent_grpc::release_agent_server::ReleaseAgentServer;
use tonic::transport::Server;
use tracing::info;

#[derive(Debug)]
pub struct App<R: ReleaseAgent + Clone> {
    config: AppConfig,
    release_agent: R,
}

#[derive(Debug)]
pub struct AppConfig {
    pub grpc: String,
}

impl<T: ReleaseAgent + Clone + Sync> App<T> {
    pub fn new(config: AppConfig, release_agent: T) -> Self {
        Self {
            config,
            release_agent,
        }
    }

    pub async fn run(&self) -> Result<(), ReleaseAgentError> {
        info!("Starting release agent");
        let addr = self
            .config
            .grpc
            .parse()
            .map_err(|_| ReleaseAgentError::ConfigError)?;
        info!("Starting grpc server at {}", addr);
        Server::builder()
            .add_service(ReleaseAgentServer::new(self.release_agent.clone()))
            .serve(addr)
            .await
            .map_err(ReleaseAgentError::TransportError)
    }
}
