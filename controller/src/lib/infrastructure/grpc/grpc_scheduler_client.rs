use std::pin::Pin;
use std::sync::Arc;
use futures::lock::Mutex;
use futures::{Stream, StreamExt};
use tracing::error;
use std::error::Error;
use tonic::transport::Channel;
use tonic::{async_trait, Streaming};
use crate::domain::action::entities::action::{
    ActionRequest as DomainActionRequest,
    ActionResponse as DomainActionResponse,
    ActionResult as DomainActionResult,
    ActionStatus as DomainActionStatus
};
use crate::domain::scheduler::services::scheduler_client::SchedulerClient;

use crate::infrastructure::grpc::proto_scheduler::controller_client::ControllerClient;
use crate::infrastructure::grpc::proto_scheduler::{
    ActionRequest as ProtoActionRequest, 
    ActionResponse as ProtoActionResponse, 
    ActionResult as ProtoActionResult,
    ExecutionContext,
    RunnerType
};

impl From<ProtoActionResponse> for DomainActionResponse {
    fn from(grpc_response: ProtoActionResponse) -> Self {
        DomainActionResponse {
            action_id: grpc_response.action_id,
            log: grpc_response.log,
            result: grpc_response.result.map(|res| DomainActionResult::from(res)),
        }
    }
}

impl From<ProtoActionResult> for DomainActionResult {
    fn from(grpc_result: ProtoActionResult) -> Self {
        DomainActionResult {
            completion: DomainActionStatus::from_i32(grpc_result.completion),
            exit_code: grpc_result.exit_code,
        }
    }
}

impl DomainActionStatus {
    pub fn from_i32(value: i32) -> DomainActionStatus {
        match value {
            0 => DomainActionStatus::Pending,
            1 => DomainActionStatus::Running,
            2 => DomainActionStatus::Completed,
            3 => DomainActionStatus::Error,
            _ => DomainActionStatus::Error,
        }
    }
}

impl From<DomainActionRequest> for ProtoActionRequest {
    fn from(domain_request: DomainActionRequest) -> Self {
        ProtoActionRequest {
            action_id: domain_request.action_id,
            context: Some(ExecutionContext {
                r#type: RunnerType::Docker as i32,
                container_image: domain_request.context.container_image.clone(),
            }),
            commands: domain_request.commands.clone(),
            repo_url: domain_request.repo_url.clone(),
        }
    }
}

pub struct GrpcSchedulerClient {
    client: Arc<Mutex<ControllerClient<Channel>>>,
}

impl GrpcSchedulerClient {
    pub async fn new(grpc_url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let client = ControllerClient::connect(grpc_url.to_string()).await?;
        tracing::info!("Connected to scheduler at {}", grpc_url);
        Ok(Self {
            client: Arc::new(Mutex::new(client)),
        })  
    }
}

#[async_trait]
impl SchedulerClient for GrpcSchedulerClient {
    async fn schedule_action(
        &self,
        request: DomainActionRequest,
    ) -> Result<
        Pin<Box<dyn Stream<Item = Result<DomainActionResponse, Box<dyn Error + Send + Sync>>> + Send>>,
        Box<dyn Error + Send + Sync>,
    > {
        let grpc_request: ProtoActionRequest = request.into();

        let mut client = self.client.lock().await;
        let response = client.schedule_action(grpc_request).await;

        let mut grpc_stream: Streaming<ProtoActionResponse> = match response {
            Ok(resp) => resp.into_inner(),
            Err(e) => {
                error!("Error while sending action to scheduler: {:?}", e);
                return Err(Box::new(e) as Box<dyn Error + Send + Sync>);
            }
        };

        let stream = async_stream::stream! {
            while let Some(result) = grpc_stream.next().await {
                match result {
                    Ok(grpc_response) => {
                        yield Ok(DomainActionResponse::from(grpc_response));
                    }
                    Err(e) => {
                        error!("Error while receiving message from scheduler: {:?}", e);
                        yield Err(Box::new(e) as Box<dyn Error + Send + Sync>);
                    }
                }
            }
        };
        Ok(Box::pin(stream))
    }
}
