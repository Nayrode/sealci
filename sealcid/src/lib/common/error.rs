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
            Error::Unhandled => write!(f, "Unhandled error"),
            Error::RestartAgentError(e) => write!(f, "Restart agent error: {}", e),
            Error::RestartControllerError(e) => write!(f, "Restart controller error: {}", e),
            Error::RestartMonitorError(e) => write!(f, "Restart monitor error: {}", e),
            Error::ToggleAgentError(e) => write!(f, "Toggle agent error: {}", e),
            Error::ToggleControllerError(e) => write!(f, "Toggle controller error: {}", e),
            Error::Unhandled => "An unhandled error occurred".to_string(),
            Error::RestartAgentError(e) => format!("Failed to restart agent: {}", e),
            Error::ToggleAgentError(e) => format!("Failed to toggle agent: {}", e),
        }
    }
}
