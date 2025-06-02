use async_trait::async_trait;

use crate::domain::pipeline::entities::pipeline::{ManifestPipeline, Pipeline, PipelineError};

#[async_trait]
pub trait PipelineService: Send + Sync {
    async fn find_all(&self, verbose: bool) -> Result<Vec<Pipeline>, PipelineError>;
    async fn find_by_id(&self, pipeline_id: i64) -> Result<Pipeline, PipelineError>;
    async fn create_pipeline(
        &self,
        repository_url: String,
        name: String,
    ) -> Result<Pipeline, PipelineError>;
    async fn create_manifest_pipeline(
        &self,
        manifest: ManifestPipeline,
        repository_url: String,
    ) -> Result<Pipeline, PipelineError>;
    async fn add_verbose_details(&self, pipeline: &mut Pipeline) -> Result<(), PipelineError>;
}
