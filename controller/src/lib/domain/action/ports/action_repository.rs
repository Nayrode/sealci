use async_trait::async_trait;
use crate::domain::action::entities::action::{Action, ActionError, ActionType};


#[async_trait]
pub trait ActionRepository: Send + Sync {
    async fn find_by_pipeline_id(&self, pipeline_id: i64) -> Result<Vec<Action>, ActionError>;
    async fn find_by_id(&self, action_id: i64) -> Result<Action, ActionError>;
    async fn create(
        &self,
        pipeline_id: i64,
        name: String,
        container_uri: String,
        r#type: ActionType,
        status: String,
    ) -> Result<Action, ActionError>;
    async fn update_status(
        &self,
        action_id: i64,
        status: &String,
    ) -> Result<Action, ActionError>;
    async fn append_log(&self, action_id: i64, log: String) -> Result<(), ActionError>;
}
