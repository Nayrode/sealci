use core::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use thiserror::Error;

use crate::domain::log::entities::log::Log;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum ActionType {
    Container,
}

impl Serialize for ActionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            ActionType::Container => serializer.serialize_str("Container"),
        }
    }
}

impl fmt::Display for ActionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ActionType::Container => write!(f, "container"),
        }
    }
}

impl FromStr for ActionType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "container" => Ok(ActionType::Container),
            _ => Err(()),
        }
    }
}

impl From<String> for ActionType {
    fn from(s: String) -> Self {
        ActionType::from_str(&s).unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ActionStatus {
    Pending,
    Running,
    Completed,
    Error,
}

impl ActionStatus {
    pub fn as_proto_name(&self) -> &'static str {
        match self {
            ActionStatus::Pending   => "ACTION_STATUS_PENDING",
            ActionStatus::Running   => "ACTION_STATUS_RUNNING",
            ActionStatus::Completed => "ACTION_STATUS_COMPLETED",
            ActionStatus::Error     => "ACTION_STATUS_ERROR",
        }
    }
}

impl fmt::Display for ActionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_proto_name())
    }
}

impl FromStr for ActionStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "ACTION_STATUS_PENDING"   | "Pending"   => Ok(ActionStatus::Pending),
            "ACTION_STATUS_RUNNING"   | "Running"   | "Scheduled" => Ok(ActionStatus::Running),
            "ACTION_STATUS_COMPLETED" | "Completed" => Ok(ActionStatus::Completed),
            "ACTION_STATUS_ERROR"     | "Error"     => Ok(ActionStatus::Error),
            _ => Err(()),
        }
    }
}

impl From<String> for ActionStatus {
    fn from(s: String) -> Self {
        ActionStatus::from_str(&s).unwrap_or(ActionStatus::Error)
    }
}

#[derive(Debug, Clone)]
pub struct ActionRequest {
    pub action_id: u32,
    pub commands: Vec<String>,
    pub context: ExecutionContext,
    pub repo_url: String,
}

#[derive(Debug, Clone)]
pub struct ExecutionContext {
    pub r#type: i32,
    pub container_image: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ActionResponse {
    pub action_id: u32,
    pub log: String,
    pub result: Option<ActionResult>,
}

#[derive(Debug, Clone)]
pub struct ActionResult {
    pub completion: ActionStatus,
    pub exit_code: Option<i32>,
}

#[derive(Debug, Clone, Deserialize, Serialize, FromRow)]
pub struct Action {
    pub id: i64,
    pub pipeline_id: i64,
    pub name: String,
    pub r#type: ActionType,
    pub container_uri: String,
    #[sqlx(default)]
    pub commands: Vec<String>,
    pub status: ActionStatus,
    pub logs: Option<Vec<String>>,
}

impl Action {
    pub fn new(
        id: i64,
        pipeline_id: i64,
        name: String,
        status: ActionStatus,
        r#type: ActionType,
        container_uri: String,
        commands: Vec<String>,
        logs: Vec<String>,
    ) -> Self {
        Self {
            id,
            pipeline_id,
            name,
            status,
            r#type,
            container_uri,
            commands,
            logs: Some(logs),
        }
    }
}

#[derive(Debug, Error)]
pub enum ActionError {
    #[error("Error while creating action: {0}")]
    CreateError(String),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Invalid input: {0}")]
    InvalidStatus(String),
    #[error("Invalid input: {0}")]
    InvalidType(String),
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ActionDTO {
    pub action_id: i64,
    pub pipeline_id: i64,
    pub name: String,
    pub action_type: String,
    pub container_uri: String,
    pub status: String,
    pub command: Option<String>,
    pub command_id: Option<i64>,
}
