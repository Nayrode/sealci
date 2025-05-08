use crate::{
    application::ports::{action_service::ActionService, scheduler_service::SchedulerService}, domain::{pipeline::ports::pipeline_repository::{self, PipelineRepository}, scheduler::{entities::scheduler::SchedulerError, services::scheduler_client::SchedulerClient}},
};
use crate::domain::action::entities::action::{
    ActionRequest as DomainActionRequest,
    ExecutionContext as ActionContext
}; 
use async_trait::async_trait;
use futures::lock::Mutex;
use tokio_stream::StreamExt;
use tracing::info;
use std::sync::Arc;

pub struct SchedulerServiceImpl {
    action_service: Arc<Box<dyn ActionService + Send + Sync>>,
    scheduler_client: Arc<Mutex<Box<dyn SchedulerClient + Send + Sync>>>,
    pipeline_repository: Arc<dyn PipelineRepository + Send + Sync>
}

impl SchedulerServiceImpl {
    pub fn new(
        action_service: Arc<Box<dyn ActionService + Send + Sync>>, 
        scheduler_client: Arc<Mutex<Box<dyn SchedulerClient + Send + Sync>>>,
        pipeline_repository: Arc<dyn PipelineRepository + Send + Sync>
    ) -> Self {
        Self { action_service, scheduler_client, pipeline_repository }
    }
}

#[async_trait]
impl SchedulerService for SchedulerServiceImpl {
    async fn execute_pipeline(&self, pipeline_id: i64) -> Result<(), SchedulerError> {
        // 1. Find actions by pipeline ID
        let mut actions = self.action_service
            .find_by_pipeline_id(pipeline_id)
            .await
            .map_err(|e| SchedulerError::Error(format!("Failed to find actions: {}", e)))?;
        
        let pipeline = self.pipeline_repository.find_by_id(pipeline_id).await.map_err(|e| SchedulerError::Error(format!("Failed to find pipeline: {}", e)))?;
        let repo_url = pipeline.repository_url.clone();
        // 2. Sort actions by ID
        actions.sort_by_key(|action| action.id);
        
        // 3. Lock the scheduler client
        let scheduler_client = self.scheduler_client.lock().await;
        
        // 4. Execute each action
        for action in actions {
            info!("Scheduling action: {:?}", action.id);
            
            // 5. Request creation to schedule the action
            let action_request = DomainActionRequest {
                action_id: action.id as u32,
                context: ActionContext {
                    r#type: action.r#type as i32,
                    container_image: Some(action.container_uri.clone()),
                },
                commands: action.commands.clone(),
                repo_url: repo_url.clone(),
            };
            
            // 6.Send the request to the scheduler and get a stream of responses
            let mut response_stream = scheduler_client
                .schedule_action(action_request)
                .await
                .map_err(|e| SchedulerError::Error(format!("Failed to schedule action: {}", e)))?;
            
            // 7. Treat the response stream
            while let Some(response) = response_stream.next().await {
                match response {
                    Ok(action_response) => {
                        // 8. Update the action status in the database
                        if let Some(result) = &action_response.result {
                            self.action_service
                                .update_status(action_response.action_id as i64, &result.completion.to_string())
                                .await
                                .map_err(|e| SchedulerError::Error(format!("Failed to update action: {}", e)))?;
                        }
                        // Stocker les logs si nécessaire
                    },
                    Err(e) => return Err(SchedulerError::Error(format!("Error from scheduler: {}", e))),
                }
            }
        }
        
        Ok(())
    }
}
