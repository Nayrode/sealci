use std::fmt::Display;

pub enum Error {
    Unhandled,
    RestartAgentError(agent::models::error::Error),
    ToggleAgentError(agent::models::error::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Unhandled => write!(f, "Unhandled error"),
            Error::RestartAgentError(_) => write!(f, "Restart agent error"),
            Error::ToggleAgentError(_) => write!(f, "Toggle agent error"),
        }
    }
}
