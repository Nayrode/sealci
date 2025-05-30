use std::sync::Arc;

use async_trait::async_trait;
use futures::lock::Mutex;
use tracing::info;

use crate::{
    application::ports::{action_service::ActionService, pipeline_service::PipelineService, scheduler_service::SchedulerService}, domain::{action::entities::action::{ActionStatus, ActionType}, log::ports::log_repository::LogRepository, pipeline::{entities::pipeline::{ManifestPipeline, Pipeline, PipelineError}, ports::pipeline_repository::PipelineRepository}}, infrastructure::repositories::{log_repository::PostgresLogRepository, pipeline_repository::PostgresPipelineRepository}
};

use super::{action_service::DefaultActionServiceImpl, scheduler_service_impl::DefaultSchedulerServiceImpl};

pub type DefaultPipelineServiceImpl = PipelineServiceImpl<PostgresPipelineRepository, PostgresLogRepository, DefaultActionServiceImpl, DefaultSchedulerServiceImpl>;

pub struct PipelineServiceImpl<R, L, A, S>
where
    R: PipelineRepository + Send + Sync,
    L: LogRepository + Send + Sync,
    A: ActionService + Send + Sync,
    S: SchedulerService + Send + Sync,
{
    repository: Arc<R>,
    logs_repository: Arc<L>,
    action_service: Arc<A>,
    scheduler_service: Arc<Mutex<S>>
}

impl<R, L, A, S> PipelineServiceImpl<R, L, A, S>
where
    R: PipelineRepository + Send + Sync,
    L: LogRepository + Send + Sync,
    A: ActionService + Send + Sync,
    S: SchedulerService + Send + Sync,
{
    pub fn new(
        repository: Arc<R>,
        logs_repository: Arc<L>,
        action_service: Arc<A>,
        scheduler_service: Arc<Mutex<S>>,
    ) -> Self {
        Self {
            repository,
            logs_repository,
            action_service,
            scheduler_service,
        }
    }
}

#[async_trait]
impl<R, L, A, S> PipelineService for PipelineServiceImpl<R, L, A, S>
where
    R: PipelineRepository + Send + Sync,
    L: LogRepository + Send + Sync,
    A: ActionService + Send + Sync,
    S: SchedulerService + Send + Sync,
{
    async fn find_all(&self, verbose: bool) -> Result<Vec<Pipeline>, PipelineError> {
        let mut pipelines = self.repository.find_all().await?;
    
        if verbose {
            for pipeline in &mut pipelines {
                self.add_verbose_details(pipeline).await?;
            }
        }
    
        Ok(pipelines)
    }

    async fn create_pipeline(
        &self,
        repository_url: String,
        name: String,
    ) -> Result<Pipeline, PipelineError> {
        self.repository.create(repository_url, name).await
    }

    async fn find_by_id(&self, pipeline_id: i64) -> Result<Pipeline, PipelineError> {
        self.repository.find_by_id(pipeline_id).await
    }

    async fn create_manifest_pipeline(
        &self,
        manifest: ManifestPipeline,
        repository_url: String,
    ) -> Result<Pipeline, PipelineError> {
        let pipeline = self.create_pipeline(repository_url, manifest.name).await?;

        for (action_name, action_data) in manifest.actions.actions.iter() {
            let _action = self
                .action_service
                .create(
                    pipeline.id,
                    action_name.to_owned(),
                    action_data.configuration.container.clone(),
                    ActionType::Container,
                    ActionStatus::Pending.to_string(),
                    Some(action_data.commands.clone()),
                )
                .await;
        }

        self.scheduler_service.lock().await.execute_pipeline(pipeline.id).await.map_err(|e| PipelineError::CreateError(e.to_string()))?;

        Ok(pipeline)
    }

    async fn add_verbose_details(&self, pipeline: &mut Pipeline) -> Result<(), PipelineError> {
        for action in &mut pipeline.actions {
            info!("Fetching verbose details for action: {:?}", action);
    
            match self.logs_repository.find_by_action_id(action.id).await {
                Ok(logs) => {
                    action.logs = Some(logs.into_iter().collect());
                }
                Err(e) => {
                    return Err(PipelineError::CreateError(format!("Error fetching logs for action {}: {}", action.name, e)));
                }
            }
        }
        Ok(())
    }
}
