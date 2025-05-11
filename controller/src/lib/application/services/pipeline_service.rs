use std::sync::Arc;

use async_trait::async_trait;
use futures::lock::Mutex;

use crate::{
    application::ports::{action_service::ActionService, pipeline_service::PipelineService, scheduler_service::SchedulerService}, domain::{action::entities::action::{ActionStatus, ActionType}, pipeline::{entities::pipeline::{ManifestPipeline, Pipeline, PipelineError}, ports::pipeline_repository::PipelineRepository}}, infrastructure::repositories::pipeline_repository::PostgresPipelineRepository
};

use super::{action_service::DefaultActionServiceImpl, scheduler_service_impl::DefaultSchedulerServiceImpl};

pub type DefaultPipelineServiceImpl = PipelineServiceImpl<PostgresPipelineRepository, DefaultActionServiceImpl, DefaultSchedulerServiceImpl>;

pub struct PipelineServiceImpl<R, A, S>
where
    R: PipelineRepository + Send + Sync,
    A: ActionService + Send + Sync,
    S: SchedulerService + Send + Sync,
{
    repository: Arc<R>,
    action_service: Arc<A>,
    scheduler_service: Arc<Mutex<S>>,
}

impl<R, A, S> PipelineServiceImpl<R, A, S>
where
    R: PipelineRepository + Send + Sync,
    A: ActionService + Send + Sync,
    S: SchedulerService + Send + Sync,
{
    pub fn new(
        repository: Arc<R>,
        action_service: Arc<A>,
        scheduler_service: Arc<Mutex<S>>,
    ) -> Self {
        Self {
            repository,
            action_service,
            scheduler_service,
        }
    }
}

#[async_trait]
impl<R, A, S> PipelineService for PipelineServiceImpl<R, A, S>
where
    R: PipelineRepository + Send + Sync,
    A: ActionService + Send + Sync,
    S: SchedulerService + Send + Sync,
{
    async fn find_all(&self, verbose: bool) -> Vec<Pipeline> { // TODO handle verbose
        self.repository.find_all().await.unwrap_or_else(|_| vec![])
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
}
