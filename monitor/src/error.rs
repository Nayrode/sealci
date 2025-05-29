#[allow(dead_code)]
pub enum Error {
    RequestError(reqwest::Error),
    NoCommitFound,
    NoPullRequestFound,
    NoListenerFound,
    FaildToReadGitEvent,
    FileReadError(std::io::Error),
}
