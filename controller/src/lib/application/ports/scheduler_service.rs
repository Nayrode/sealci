use async_trait::async_trait;

use crate::domain::scheduler::entities::scheduler::SchedulerError;

#[async_trait]
pub trait SchedulerService: Send + Sync {
    async fn execute_pipeline(&self, pipeline_id: i64) -> Result<(), SchedulerError>;
}
