use std::sync::Arc;
use futures::lock::Mutex;
use crate::application::ports::{
        pipeline_service::PipelineService,
        action_service::ActionService,
        scheduler_service::SchedulerService
    };

#[derive(Clone)]
pub struct AppState {
    pub pipeline: Arc<dyn PipelineService + Send + Sync>,
    pub action: Arc<dyn ActionService + Send + Sync>,
    pub scheduler: Arc<Mutex<dyn SchedulerService + Send + Sync>>,
}

impl AppState {
    pub fn new(
        pipeline: Arc<dyn PipelineService + Send + Sync>,
        action: Arc<dyn ActionService + Send + Sync>,
        scheduler: Arc<Mutex<dyn SchedulerService + Send + Sync>>,
    ) -> Self {
        Self { pipeline, action, scheduler }
    }
}