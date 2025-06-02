use crate::domain::action::entities::action::{
    ActionRequest as DomainActionRequest, ExecutionContext,
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
        // Find all actions associated with the pipeline
        let mut actions = self
            .action_service
            .find_by_pipeline_id(pipeline_id)
            .await
            .map_err(|e| SchedulerError::Error(format!("Failed to find actions: {}", e)))?;

        // Find the pipeline to get its repository URL
        let pipeline = self
            .pipeline_repository
            .find_by_id(pipeline_id)
            .await
            .map_err(|e| SchedulerError::Error(format!("Failed to find pipeline: {}", e)))?;

        let repo_url = pipeline.repository_url.clone();
        info!(
            "Scheduling actions for pipeline {} with id {} with repository URL: {}",
            pipeline.name, pipeline_id, repo_url
        );

        // Sort actions by their IDs to ensure they are processed in the correct order
        actions.sort_by_key(|action| action.id);

        // Acquire the scheduler client lock to ensure thread safety
        let client = self.scheduler_client.lock().await;

        for action in actions {
            info!("Scheduling action {} with ID {}", action.name, action.id);

            // Prepare the action request for the scheduler gRPC
            let action_request = DomainActionRequest {
                action_id: action.id as u32,
                context: ExecutionContext {
                    r#type: action.r#type as i32,
                    container_image: Some(action.container_uri.clone()),
                },
                commands: action.commands.clone(),
                repo_url: repo_url.clone(),
            };

            // Call the scheduler client to schedule the action and get a response stream
            // A response stream is a stream of ActionResponse items
            let mut response_stream =
                client.schedule_action(action_request).await.map_err(|e| {
                    error!("Failed to schedule action {}: {:?}", action.id, e);
                    SchedulerError::Error("SchedulerError: ".into())
                })?;

            while let Some(item) = response_stream.next().await {
                // Process each item in the response stream
                match item {
                    Ok(action_response) => {
                        info!(
                            "[SCHEDULER] RESPONSE = ActionResponse {{ action_id: {}, log: {:?}, result: {:?} }}",
                            action_response.action_id,
                            action_response.log,
                            action_response.result,
                        );

                        // Update action status in the database
                        if let Some(result) = &action_response.result {
                            let status_str = result.completion.as_proto_name();
                            self.action_service
                                .update_status(
                                    action_response.action_id as i64,
                                    &status_str.to_string(),
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

                        // Append log data to the action
                        // This assumes that the action_response.log is a String
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
