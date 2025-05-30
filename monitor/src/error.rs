use std::fmt::Display;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Error {
    RequestError(reqwest::Error),
    NoCommitFound,
    NoPullRequestFound,
    NoListenerFound,
    FaildToReadGitEvent,
    FileReadError(std::io::Error),
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
        }
    }
}
