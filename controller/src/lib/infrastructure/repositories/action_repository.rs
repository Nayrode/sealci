use async_trait::async_trait;
use std::{collections::HashMap, sync::Arc};

use crate::domain::action::entities::action::{Action, ActionDTO, ActionError, ActionStatus, ActionType};
use crate::domain::action::ports::action_repository::ActionRepository;
use crate::infrastructure::db::postgres::Postgres;

pub struct PostgresActionRepository {
    pub postgres: Arc<Postgres>,
}

impl PostgresActionRepository {
    pub fn new(postgres: Arc<Postgres>) -> Self {
        Self { postgres }
    }
}

#[async_trait]
impl ActionRepository for PostgresActionRepository {
    async fn create(
        &self,
        pipeline_id: i64,
        name: String,
        container_uri: String,
        r#type: ActionType,
        status: String,
    ) -> Result<Action, ActionError> {
        let result = sqlx::query!(
      r#"INSERT INTO actions (pipeline_id, name, container_uri, type, status) VALUES ($1, $2, $3, $4, $5) RETURNING id, pipeline_id, name, container_uri, type, status"#,
      pipeline_id, name, container_uri, &r#type.to_string(), status
    )
    .fetch_one(&self.postgres.get_pool())
    .await;

        result
            .map(|row| Action {
                id: row.id,
                pipeline_id: row.pipeline_id,
                name: row.name,
                r#type: row.r#type.into(),
                container_uri: row.container_uri,
                status: row.status.into(),
                commands: vec![],
                logs: None,
            })
            .map_err(ActionError::DatabaseError)
    }

    async fn find_by_id(&self, action_id: i64) -> Result<Action, ActionError> {
        let row = sqlx::query_as!(
            ActionDTO,
            r#"
  SELECT 
    a.id as action_id,
    a.pipeline_id,
    a.name,
    a.container_uri,
    a.type,
    a.status,
    c.id as command_id,
    c.command
  FROM actions a 
  LEFT JOIN commands c ON a.id = c.action_id
  WHERE a.id = $1
  "#,
            action_id
        )
        .fetch_one(&self.postgres.get_pool())
        .await
        .map_err(ActionError::DatabaseError)?;

        let status = row
            .status
            .parse::<ActionStatus>()
            .map_err(|_| ActionError::InvalidStatus(row.status.clone()))?;

        let action_type = row
            .r#type
            .parse::<ActionType>()
            .map_err(|_| ActionError::InvalidType(row.r#type.clone()))?;

        let action = Action {
            id: row.action_id,
            pipeline_id: row.pipeline_id,
            name: row.name.clone(),
            container_uri: row.container_uri.clone(),
            r#type: action_type,
            status,
            commands: vec![row.command],
            logs: None,
        };

        Ok(action)
    }

    async fn find_by_pipeline_id(&self, pipeline_id: i64) -> Result<Vec<Action>, ActionError> {
        let rows = sqlx::query_as!(
            ActionDTO,
            r#"
    SELECT 
      a.id as action_id,
      a.pipeline_id,
      a.name,
      a.container_uri,
      a.type,
      a.status,
      c.id as command_id,
      c.command
    FROM actions a 
    LEFT JOIN commands c ON a.id = c.action_id
    WHERE a.pipeline_id = $1
    "#,
            pipeline_id
        )
        .fetch_all(&self.postgres.get_pool())
        .await
        .map_err(ActionError::DatabaseError)?;

        let mut actions_map = HashMap::new();

        for row in rows {
            let status = row
                .status
                .parse::<ActionStatus>()
                .map_err(|_| ActionError::InvalidStatus(row.status.clone()))?;

            let action_type = row
                .r#type
                .parse::<ActionType>()
                .map_err(|_| ActionError::InvalidType(row.r#type.clone()))?;

            let action_entry = actions_map.entry(row.action_id).or_insert_with(|| Action {
                id: row.action_id,
                pipeline_id: row.pipeline_id,
                name: row.name.clone(),
                container_uri: row.container_uri.clone(),
                r#type: action_type,
                status,
                commands: Vec::new(),
                logs: None,
            });

            action_entry.commands.push(row.command);
        }

        let actions: Vec<Action> = actions_map.into_values().collect();

        Ok(actions)
    }

    async fn update_status(
        &self,
        action_id: i64,
        status: &String,
    ) -> Result<Action, ActionError> {
        let result = sqlx::query!(
            r#"UPDATE actions SET status = $1 WHERE id = $2 RETURNING id, pipeline_id, name, container_uri, type, status"#,
            status,
            action_id
        )
        .fetch_one(&self.postgres.get_pool())
        .await;

        result
            .map(|row| Action {
                id: row.id,
                pipeline_id: row.pipeline_id,
                name: row.name,
                r#type: row.r#type.into(),
                container_uri: row.container_uri,
                status: row.status.into(),
                commands: vec![],
                logs: None,
            })
            .map_err(ActionError::DatabaseError)
    }

    async fn append_log(&self, action_id: i64, log: String) -> Result<(), ActionError> {
        sqlx::query!(
            r#"INSERT INTO logs (action_id, data) VALUES ($1, $2)"#,
            action_id,
            log
        )
        .execute(&self.postgres.get_pool())
        .await
        .map_err(ActionError::DatabaseError)?;
        Ok(())
    }
}
