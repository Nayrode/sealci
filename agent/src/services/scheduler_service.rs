use std::time::Duration;

use tokio::time::sleep;
use tokio_stream::StreamExt;
use tonic::IntoStreamingRequest;
use tracing::{error, warn};

use crate::{
    models::error::Error::{self, ConnectionError, RegistrationError},
    proto::{agent_client::AgentClient, HealthStatus, Hostname, RegisterAgentRequest},
};

use super::health_service::HealthService;
#[derive(Clone)]
pub struct SchedulerService {
    /// This is the client that is exposed by the scheduler for the agent.
    scheduler_agent_client: AgentClient<tonic::transport::Channel>,
    health_service: HealthService,
    /// The URL that the agent will give to the scheduler.
    agent_advertise_url: String,
    port: u32,
    agent_id: Option<u32>,
}

impl SchedulerService {
    pub async fn init(
        scheduler_url: String,
        agent_host: String,
        port: u32,
        health_service: HealthService,
    ) -> Result<Self, Error> {
        let mut retry_delay = Duration::from_secs(2);
        const MAX_RETRY_DELAY: u64 = 64;
        let mut retry_count = 0;
        let scheduler_agent_client = loop {
            match AgentClient::connect(scheduler_url.to_string())
                .await
                .map_err(ConnectionError)
            {
                Ok(client) => break client,
                Err(e) => {
                    if retry_count >= 10 {
                        return Err(e);
                    }
                    warn!(
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
        Ok(SchedulerService {
            scheduler_agent_client,
            health_service,
            agent_advertise_url: agent_host,
            port,
            agent_id: None,
        })
    }

    pub async fn register(&mut self) -> Result<(), Error> {
        let host = Hostname {
            host: self.agent_advertise_url.clone(),
            port: self.port,
        };
        let health = self.health_service.get_health().await;
        let req = RegisterAgentRequest {
            health: Some(health),
            hostname: Some(host),
        };
        let request = tonic::Request::new(req);
        let res = self
            .scheduler_agent_client
            .register_agent(request)
            .await
            .map_err(RegistrationError)?
            .into_inner();
        self.agent_id = Some(res.id);
        Ok(())
    }

    pub async fn report_health(&mut self) -> Result<(), Error> {
        let agent_id = self.agent_id.ok_or(Error::NotRegisteredError)?;
        let (health_stream, handle_health_stream) = self.health_service.get_health_stream();
        let stream = health_stream
            .map(move |health| HealthStatus {
                agent_id,
                health: Some(health),
            })
            .into_streaming_request();
        self.scheduler_agent_client
            .report_health_status(stream)
            .await
            .map_err(Error::ReportHealthError)?;
        handle_health_stream
            .await
            .map_err(|_| Error::HealthStreamError)?;
        error!("Health ended");
        Ok(())
    }
}
