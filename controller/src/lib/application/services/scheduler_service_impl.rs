use crate::domain::action::entities::action::{
    ActionRequest, ExecutionContext,
};
use crate::{
    application::ports::{action_service::ActionService, scheduler_service::SchedulerService},
    domain::{
        pipeline::ports::pipeline_repository::PipelineRepository,
        scheduler::{
            entities::scheduler::SchedulerError, services::scheduler_client::SchedulerClient,
        },
    },
    infrastructure::{
        grpc::grpc_scheduler_client::GrpcSchedulerClient,
        repositories::pipeline_repository::PostgresPipelineRepository,
    },
};
use async_trait::async_trait;
use futures::lock::Mutex;
use std::sync::Arc;
use tokio_stream::StreamExt;
use tracing::{error, info};

use super::action_service::DefaultActionServiceImpl;

pub type DefaultSchedulerServiceImpl =
    SchedulerServiceImpl<DefaultActionServiceImpl, GrpcSchedulerClient, PostgresPipelineRepository>;

pub struct SchedulerServiceImpl<A, S, R>
where
    A: ActionService + Send + Sync,
    S: SchedulerClient + Send + Sync,
    R: PipelineRepository + Send + Sync,
{
    action_service: Arc<A>,
    scheduler_client: Arc<Mutex<S>>,
    pipeline_repository: Arc<R>,
}

impl<A, S, R> SchedulerServiceImpl<A, S, R>
where
    A: ActionService + Send + Sync,
    S: SchedulerClient + Send + Sync,
    R: PipelineRepository + Send + Sync,
{
    pub fn new(
        action_service: Arc<A>,
        scheduler_client: Arc<Mutex<S>>,
        pipeline_repository: Arc<R>,
    ) -> Self {
        Self {
            action_service,
            scheduler_client,
            pipeline_repository,
        }
    }
}

#[async_trait]
impl<A, S, R> SchedulerService for SchedulerServiceImpl<A, S, R>
where
    A: ActionService + Send + Sync,
    S: SchedulerClient + Send + Sync,
    R: PipelineRepository + Send + Sync,
{
    async fn execute_pipeline(&self, pipeline_id: i64) -> Result<(), SchedulerError> {
        let mut actions = self
            .action_service
            .find_by_pipeline_id(pipeline_id)
            .await
            .map_err(|e| SchedulerError::Error(format!("Failed to find actions: {}", e)))?;

        let pipeline = self
            .pipeline_repository
            .find_by_id(pipeline_id)
            .await
            .map_err(|e| SchedulerError::Error(format!("Failed to find pipeline: {}", e)))?;
        let repo_url = pipeline.repository_url.clone();

        actions.sort_by_key(|action| action.id);

        let client = self.scheduler_client.lock().await;

        for action in actions {
            info!("Scheduling action: {}", action.id);

            let action_request = ActionRequest {
                action_id: action.id as u32,
                context: ExecutionContext {
                    r#type: action.r#type as i32,
                    container_image: Some(action.container_uri.clone()),
                },
                commands: action.commands.clone(),
                repo_url: repo_url.clone(),
            };

            let mut response_stream = client
                .schedule_action(action_request)
                .await
                .map_err(|e| {
                    error!("Failed to schedule action {}: {:?}", action.id, e);
                    SchedulerError::Error(format!("Failed to schedule action: {}", e))
                })?;

            while let Some(item) = response_stream.next().await {
                match item {
                    Ok(action_response) => {
                        if let Some(result) = &action_response.result {
                            self.action_service
                                .update_status(
                                    action_response.action_id as i64,
                                    &result.completion.to_string(),
                                )
                                .await
                                .map_err(|e| {
                                    error!(
                                        "Failed to update action {} status: {:?}",
                                        action_response.action_id, e
                                    );
                                    SchedulerError::Error(format!("Failed to update action: {}", e))
                                })?;
                        }

                        let log_data = action_response.log.clone();
                        self.action_service
                            .append_log(action_response.action_id as i64, log_data)
                            .await
                            .map_err(|e| {
                                error!(
                                    "Failed to store log for action {}: {:?}",
                                    action_response.action_id, e
                                );
                                SchedulerError::Error(format!("Failed to store log: {}", e))
                            })?;
                    }
                    Err(e) => {
                        error!(
                            "Error from scheduler stream for action {}: {:?}",
                            action.id, e
                        );
                        return Err(SchedulerError::Error(format!(
                            "Error from scheduler: {}",
                            e
                        )));
                    }
                }
            }
        }

        Ok(())
    }
}
