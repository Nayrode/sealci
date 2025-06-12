use std::fmt::Display;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Error {
    Error(String),
    RequestError(reqwest::Error),
    NoCommitFound,
    NoPullRequestFound,
    NoListenerFound,
    NoTagFound,
    FaildToReadGitEvent,
    FileReadError(std::io::Error),
    ServerError(std::io::Error),
    ServerUninitialized,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::RequestError(e) => write!(f, "Request error: {}", e),
            Error::NoCommitFound => write!(f, "No commit found"),
            Error::NoPullRequestFound => write!(f, "No pull request found"),
            Error::NoListenerFound => write!(f, "No listener found"),
            Error::FaildToReadGitEvent => write!(f, "Failed to read Git event"),
            Error::FileReadError(e) => write!(f, "File read error: {}", e),
            Error::Error(e) => write!(f, "Error: {}", e),
            Error::ServerError(e) => write!(f, "Server error: {}", e),
            Error::NoTagFound => write!(f, "No tag found"),
            Error::ServerUninitialized => write!(
                f,
                "You tried to launch the monitor server but i was nerver initialized"
            ),
        }
    }
}
