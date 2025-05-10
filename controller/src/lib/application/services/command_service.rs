use crate::{
    application::ports::command_service::CommandService, domain::command::{entities::command::{Command, CommandError}, ports::command_repository::CommandRepository},
};
use async_trait::async_trait;
use std::sync::Arc;

pub struct CommandServiceImpl<R> where R: CommandRepository + Send + Sync {
    repository: Arc<R>,
}

impl<R> CommandServiceImpl<R> where R: CommandRepository + Send + Sync {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<R> CommandService for CommandServiceImpl<R> where R: CommandRepository + Send + Sync {
    async fn find_by_action_id(&self, action_id: i64) -> Result<Vec<Command>, CommandError> {
        self.repository.find_by_action_id(action_id).await
    }

    async fn create(&self, action_id: i64, command: String) -> Result<Command, CommandError> {
        self.repository.create(action_id, command).await
    }
}
