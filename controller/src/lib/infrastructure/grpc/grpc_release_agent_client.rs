use async_trait::async_trait;
use std::sync::Arc;

use futures::lock::Mutex;
use tonic::{transport::Channel, Response};

use crate::domain::{
    self,
    releases::entities::{CreateReleaseRequest, CreateReleaseResponse, PublicKey, ReleaseStatus},
};

use crate::infrastructure::grpc::proto_release_agent::{
    CreateReleaseRequest as ProtoCreateReleaseRequest,
    CreateReleaseResponse as ProtoCreateReleaseResponse,
    CreateReleaseStatus as ProtoCreateReleaseStatus, PublicKey as ProtoPublicKey,
};

use super::proto_release_agent::release_agent_client::ReleaseAgentClient;

pub struct GrpcReleaseAgentClient {
    client: Arc<Mutex<ReleaseAgentClient<Channel>>>,
}

impl GrpcReleaseAgentClient {
    pub async fn new(grpc_url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let client = ReleaseAgentClient::connect(grpc_url.to_string()).await?;
        tracing::info!("Connected to release agent at {}", grpc_url);
        Ok(Self {
            client: Arc::new(Mutex::new(client)),
        })
    }
}

impl From<ProtoPublicKey> for PublicKey {
    fn from(grpc_key: ProtoPublicKey) -> Self {
        PublicKey {
            key_data: grpc_key.key_data,
            fingerprint: grpc_key.fingerprint,
        }
    }
}

impl From<ProtoCreateReleaseStatus> for ReleaseStatus {
    fn from(grpc_status: ProtoCreateReleaseStatus) -> Self {
        match grpc_status {
            ProtoCreateReleaseStatus::Failure => Self::FAILURE,
            ProtoCreateReleaseStatus::Success => Self::SUCCESS,
        }
    }
}

impl From<CreateReleaseRequest> for ProtoCreateReleaseRequest {
    fn from(grpc_request: CreateReleaseRequest) -> Self {
        ProtoCreateReleaseRequest {
            repo_url: grpc_request.repo_url,
            revision: grpc_request.revision,
        }
    }
}

impl From<Response<ProtoCreateReleaseResponse>> for CreateReleaseResponse {
    fn from(grpc_response: Response<ProtoCreateReleaseResponse>) -> Self {
        let grpc_response: ProtoCreateReleaseResponse = grpc_response.into_inner();
        let status = match grpc_response.status() {
            ProtoCreateReleaseStatus::Failure => ReleaseStatus::FAILURE,
            ProtoCreateReleaseStatus::Success => ReleaseStatus::SUCCESS,
        };
        let key = match grpc_response.public_key {
            Some(public_key) => Some(PublicKey {
                fingerprint: public_key.fingerprint,
                key_data: public_key.key_data,
            }),
            None => None,
        };
        CreateReleaseResponse {
            status: status,
            release_id: grpc_response.release_id,
            public_key: key,
        }
    }
}

#[async_trait]
impl domain::releases::services::ReleaseAgentClient for GrpcReleaseAgentClient {
    async fn release(
        &self,
        request: CreateReleaseRequest,
    ) -> Result<CreateReleaseResponse, Box<dyn std::error::Error>> {
        let grpc_request: ProtoCreateReleaseRequest = request.into();
        let mut client = self.client.lock().await;
        let response: CreateReleaseResponse = client.create_release(grpc_request).await?.into();
        Ok(response)
    }
}
