use crate::{
    application::ports::{action_service::ActionService, command_service::CommandService}, domain::action::{entities::action::{Action, ActionError, ActionType}, ports::action_repository::ActionRepository}, infrastructure::repositories::action_repository::PostgresActionRepository,

};
use async_trait::async_trait;
use std::sync::Arc;

use super::command_service::DefaultCommandServiceImpl;

pub type DefaultActionServiceImpl = ActionServiceImpl<PostgresActionRepository, DefaultCommandServiceImpl>;

pub struct ActionServiceImpl<R, C> where R: ActionRepository + Send + Sync, C: CommandService + Send + Sync {
    repository: Arc<R>,
    command_service: Arc<C>
}

impl<R, C> ActionServiceImpl<R, C> where R: ActionRepository + Send + Sync, C: CommandService + Send + Sync {
    pub fn new(
        repository: Arc<R>,
        command_service: Arc<C>,
    ) -> Self {
        Self {
            repository,
            command_service,
        }
    }
}

#[async_trait]
impl<R, C> ActionService for ActionServiceImpl<R, C> where R: ActionRepository + Send + Sync, C: CommandService + Send + Sync {
    async fn create(
        &self,
        pipeline_id: i64,
        name: String,
        container_uri: String,
        r#type: ActionType,
        status: String,
        commands: Option<Vec<String>>,
    ) -> Result<Action, ActionError> {
        let action = self
            .repository
            .create(pipeline_id, name, container_uri, r#type, status)
            .await?;

        if let Some(commands_vec) = commands {
            for command_str in commands_vec {
                let _command = self.command_service.create(action.id, command_str).await;
            }
        }

        Ok(action)
    }

    async fn find_by_id(&self, action_id: i64) -> Result<Action, ActionError> {
        self.repository.find_by_id(action_id).await
    }

    async fn find_by_pipeline_id(&self, pipeline_id: i64) -> Result<Vec<Action>, ActionError> {
        self.repository.find_by_pipeline_id(pipeline_id).await
    }

    async fn update_status(
        &self,
        action_id: i64,
        status: &String,
    ) -> Result<Action, ActionError> {
        self.repository.update_status(action_id, status).await
    }
    
}
