use std::fmt::Display;

pub enum Error {
    Unhandled,
    RestartAgentError(agent::models::error::Error),
    RestartControllerError(controller::application::AppError),
    RestartMonitorError(monitor::error::Error),
    ToggleAgentError(agent::models::error::Error),
    ToggleControllerError(controller::application::AppError),
    ToggleMonitorError(monitor::error::Error),
}

impl Into<String> for Error {
    fn into(self) -> String {
        match self {
            Error::Unhandled => "An unhandled error occurred".to_string(),
            Error::RestartAgentError(e) => format!("Failed to restart agent: {}", e),
            Error::RestartControllerError(e) => format!("Failed to restart controller: {}", e),
            Error::RestartMonitorError(e) => format!("Failed to restart monitor: {}", e),
            Error::ToggleAgentError(e) => format!("Failed to toggle agent: {}", e),
            Error::ToggleControllerError(e) => format!("Failed to toggle controller: {}", e),
            Error::ToggleMonitorError(e) => format!("Failed to toggle monitor: {}", e),
        }
    }
}
