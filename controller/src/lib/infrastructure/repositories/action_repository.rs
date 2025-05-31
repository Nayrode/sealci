use async_trait::async_trait;
use std::{collections::HashMap, sync::Arc};

use crate::domain::action::entities::action::{
    Action, ActionDTO, ActionError, ActionStatus, ActionType,
};
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
        let rows = sqlx::query_as!(
            ActionDTO,
            r#"
            SELECT
                a.id            AS action_id,
                a.pipeline_id,
                a.name,
                a.type          AS action_type,
                a.container_uri,
                a.status,
                c.command       AS "command?",
                c.id            AS "command_id?"
            FROM actions a
            LEFT JOIN commands c ON a.id = c.action_id
            WHERE a.id = $1
            "#,
            action_id
        )
        .fetch_all(&self.postgres.get_pool())
        .await
        .map_err(ActionError::DatabaseError)?;

        if rows.is_empty() {
            return Err(ActionError::DatabaseError(sqlx::Error::RowNotFound));
        }

        let first = &rows[0];
        let status = first
            .status
            .parse::<ActionStatus>()
            .map_err(|_| ActionError::InvalidStatus(first.status.clone()))?;
        let action_type = first
            .action_type
            .parse::<ActionType>()
            .map_err(|_| ActionError::InvalidType(first.action_type.clone()))?;

        let mut commands_vec = Vec::new();
        for r in &rows {
            if let Some(cmd) = r.command.clone() {
                commands_vec.push(cmd);
            }
        }

        Ok(Action {
            id: first.action_id,
            pipeline_id: first.pipeline_id,
            name: first.name.clone(),
            r#type: action_type,
            container_uri: first.container_uri.clone(),
            status,
            commands: commands_vec,
            logs: None,
        })
    }

    async fn find_by_pipeline_id(&self, pipeline_id: i64) -> Result<Vec<Action>, ActionError> {
        let rows = sqlx::query_as!(
            ActionDTO,
            r#"
            SELECT
                a.id            AS action_id,
                a.pipeline_id,
                a.name,
                a.type          AS action_type,
                a.container_uri,
                a.status,
                c.command       AS "command?",
                c.id            AS "command_id?"
            FROM   actions  a
            LEFT   JOIN commands c ON a.id = c.action_id
            WHERE  a.pipeline_id = $1
            "#,
            pipeline_id
        )
        .fetch_all(&self.postgres.get_pool())
        .await
        .map_err(ActionError::DatabaseError)?;

        let mut map: HashMap<i64, Action> = HashMap::new();
        for row in rows {
            let status = row
                .status
                .parse::<ActionStatus>()
                .map_err(|_| ActionError::InvalidStatus(row.status.clone()))?;

            let ty = row
                .action_type
                .parse::<ActionType>()
                .map_err(|_| ActionError::InvalidType(row.action_type.clone()))?;

            let entry = map.entry(row.action_id).or_insert_with(|| Action {
                id: row.action_id,
                pipeline_id: row.pipeline_id,
                name: row.name.clone(),
                r#type: ty,
                container_uri: row.container_uri.clone(),
                status,
                commands: Vec::new(),
                logs: None,
            });

            if let Some(cmd) = row.command {
                entry.commands.push(cmd);
            }
        }

        let mut actions: Vec<Action> = map.into_values().collect();
        actions.sort_by_key(|a| a.id);
        Ok(actions)
    }

    async fn update_status(&self, action_id: i64, status: &String) -> Result<Action, ActionError> {
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
