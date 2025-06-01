use actix_multipart::form::{tempfile::TempFile, text::Text, MultipartForm};

use crate::error::Error;

#[derive(PartialEq, serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub enum GitEvent {
    #[default]
    Commit,
    PullRequest,
    Tag,
    All,
}

impl TryFrom<String> for GitEvent {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "commit" => Ok(GitEvent::Commit),
            "pull_request" => Ok(GitEvent::PullRequest),
            "tag" => Ok(GitEvent::Tag),
            "all" | "*" => Ok(GitEvent::All),
            _ => Err(Error::FaildToReadGitEvent),
        }
    }
}

#[derive(Debug, MultipartForm)]
pub struct CreateConfigForm {
    pub actions_file: TempFile,
    pub events: Text<GitEvent>,
    pub repository_owner: Text<String>,
    pub repository_name: Text<String>,
    pub github_token: Text<String>,
}

#[derive(Debug, MultipartForm)]
pub struct UpdateConfigForm {
    pub actions_file: Option<TempFile>,
    pub events: Option<Text<Vec<GitEvent>>>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub struct GitTag {
    pub name: String,
}

impl PartialEq for GitTag {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

