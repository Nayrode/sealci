//use scheduler::proto::agent as agent;
use sealci_scheduler::proto::scheduler as agent;
use agent::agent_client::AgentClient;
use agent::agent_server::AgentServer;
use agent::{HealthStatus, Health, Empty};

//use scheduler::proto::controller as controller;
use sealci_scheduler::proto::scheduler as controller;
use controller::controller_server::ControllerServer;

use sealci_scheduler::interfaces::server as server;
use server::agent_interface::AgentService;
use server::controller_interface::ControllerService;

use sealci_scheduler::logic as logic;
use logic::agent_pool_logic::AgentPool;
// use logic::action_queue_logic::ActionsQueue;

use tonic::transport::Server;
use tonic::transport::Channel;
use tonic::Request;
use tokio_stream::iter;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::Duration;

#[tokio::test]
async fn test_report_health_status() -> Result<(), Box<dyn Error>> {
    tokio::spawn(async {
        let addr = "[::1]:50051".parse().unwrap();
        let agent_pool = Arc::new(Mutex::new(AgentPool::new()));
        // let action_queue: Arc<Mutex<ActionsQueue>> = Arc::new(Mutex::new(ActionsQueue::new()));
        let agent = AgentService::new(agent_pool.clone());
        let controller = ControllerService::new(agent_pool.clone());
        let service = tonic_reflection::server::Builder::configure()
            .register_encoded_file_descriptor_set(sealci_scheduler::proto::FILE_DESCRIPTOR_SET)
            .build_v1()
            .unwrap();

        Server::builder()
            .add_service(service)
            .add_service(AgentServer::new(agent))
            .add_service(ControllerServer::new(controller))
            .serve(addr)
            .await
            .unwrap();
    });

    tokio::time::sleep(Duration::from_secs(1)).await;

    let channel = Channel::from_static("http://[::1]:50051").connect().await?;
    let mut client = AgentClient::new(channel);

    let health_status1 = HealthStatus {
        agent_id: 1,
        health: Some(Health { cpu_avail: 80, memory_avail: 512 }),
    };

    let health_status2 = HealthStatus {
        agent_id: 2,
        health: Some(Health { cpu_avail: 60, memory_avail: 1024 }),
    };

    let health_status3 = HealthStatus {
        agent_id: 3,
        health: None,
    };

    let health_status_stream = iter(vec![health_status1, health_status2, health_status3]);

    let response = client.report_health_status(Request::new(health_status_stream)).await?;

    assert_eq!(response.get_ref(), &Empty {});

    Ok(())
}
