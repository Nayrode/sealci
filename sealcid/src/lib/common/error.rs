use std::fmt::Display;

pub enum Error {
    Unhandled,
    RestartAgentError(agent::models::error::Error),
    ToggleAgentError(agent::models::error::Error),
}

impl Into<String> for Error {
    fn into(self) -> String {
        match self {
            Error::Unhandled => "An unhandled error occurred".to_string(),
            Error::RestartAgentError(e) => format!("Failed to restart agent: {}", e),
            Error::ToggleAgentError(e) => format!("Failed to toggle agent: {}", e),
        }
    }
}
