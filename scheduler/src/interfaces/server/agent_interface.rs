use crate::errors::Error;

use crate::logic::agent_pool_logic::Agent as PoolAgent;
use crate::logic::agent_pool_logic::Hostname;
use crate::logic::agent_pool_logic::{AgentPool, compute_score};
use tracing::{info, debug, error};

use crate::proto::scheduler as proto;
use proto::agent_server::Agent;

use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_stream::StreamExt;

#[derive(Clone)]
pub struct AgentService {
    agent_pool: Arc<Mutex<AgentPool>>, // The ArcMutex is on the agent_pool, for the highest level of granularity on concurrency control
}

impl AgentService {
    pub fn new(agent_pool: Arc<Mutex<AgentPool>>) -> Self {
        Self { agent_pool }
    }
}

#[tonic::async_trait]
impl Agent for AgentService {
    async fn register_agent(
        &self,
        request: tonic::Request<proto::RegisterAgentRequest>,
    ) -> Result<tonic::Response<proto::RegisterAgentResponse>, tonic::Status> {
        // Extract the inner data from the request.
        let inner_req = request.into_inner();

        let input = inner_req.health.ok_or_else(|| {
            error!("[Scheduler]: Health status is missing in the request");
            return Error::GrpcRequestError(tonic::Status::invalid_argument(
                "Health status is missing",
            ));
        })?;

        let cpu_avail = input.cpu_avail.clone();
        let memory_avail = input.memory_avail.clone();

        debug!("[Scheduler]: Received request from Agent: {:?}", input);
        debug!(
            "\n  - Agent CPU usage: {}\n  - Agent memory usage: {}",
            cpu_avail, memory_avail
        );

        // Unwrap `hostname` or return a custom error
        let hostname = inner_req.hostname.ok_or_else(|| {
            error!("[Scheduler]: Hostname is missing in the request");
            return Error::GrpcRequestError(tonic::Status::invalid_argument("Hostname is missing"));
        })?;

        let host = hostname.host.clone();
        let port = hostname.port.clone();

        debug!("[Scheduler]: Received request from Agent: {:?}", hostname);
        debug!(
            "\n  - Agent host usage: {}\n  - Agent port usage: {}",
            host, port
        );

        // Lock the Agent Pool (to ensure thread-safe access). This is a tokio Mutex, not a standard one.
        let mut pool = self.agent_pool.lock().await;

        let id = pool.generate_unique_id();
        let score = compute_score(cpu_avail, memory_avail);
        let new_hostname = Hostname::new(host, port);

        // Create a new Agent and add it to the Pool (it gets sorted)
        let new_agent = PoolAgent::new(id, new_hostname.clone(), score);

        // Response is the newly created Agent's ID.
        let response = proto::RegisterAgentResponse {
            id: new_agent.get_id(),
        };

        info!(
            "[Scheduler]: Agent {}:{} registered successfully with ID: {}",
            new_hostname.get_host(), new_hostname.get_port(), response.id
        );

        pool.push(new_agent);

        Ok(tonic::Response::new(response))
    }

    async fn report_health_status(
        &self,
        request: tonic::Request<tonic::Streaming<proto::HealthStatus>>,
    ) -> Result<tonic::Response<proto::Empty>, tonic::Status> {
        let mut stream = request.into_inner();

        while let Some(health_status) = stream.next().await {
            let status = match health_status {
                Ok(status) => status,
                Err(e) => {
                    error!("[Scheduler]: Error receiving health status: {:?}", e);
                    return Err(tonic::Status::internal("Error receiving health status"));
                }
            };

            let health = match status.health {
                Some(health) => health,
                None => {
                    error!(
                        "[Scheduler]: Health field is missing for Agent {}",
                        status.agent_id
                    );
                    continue; // Skip to the next health status if health data is missing
                }
            };

            info!(
                "[Scheduler]: Received health status from agent {}: CPU: {}, Memory: {}",
                status.agent_id, health.cpu_avail, health.memory_avail
            );

            // Lock the Agent Pool (to ensure thread-safe access). This is a tokio Mutex, not a standard one.
            let mut pool = self.agent_pool.lock().await;

            // Find the Agent in the Pool
            let agent = match pool.find_agent_mut(status.agent_id) {
                Some(agent) => agent,
                None => {
                    error!(
                        "[Scheduler]: Agent ID {} not found in the Pool",
                        status.agent_id
                    );
                    continue; // Skip to the next health status if the agent is not found
                }
            };

            // Compute the Agent's new score and set it.
            let updated_score = compute_score(health.cpu_avail, health.memory_avail / 100_000_000); // Divide by 10^8 to have the same scale/order of magnitude as the CPU.
            agent.set_score(updated_score);

            // Check if the Agent's position in the Pool is now out of order
            let is_out_of_order = pool.check_agent_neighbors(status.agent_id);
            if is_out_of_order {
                pool.sort(); // Resort the Pool if the Agent is out of order
            }
        }

        Ok(tonic::Response::new(proto::Empty {}))
    }
}
