use models::PullRequest;

use crate::{constants::GITHUB_API_URL, error::Error};

pub mod http;
pub mod models;

pub struct GitHubClient {
    client: http::HttpClient,
}

impl GitHubClient {
    pub fn new() -> Self {
        GitHubClient {
            client: http::HttpClient::new(),
        }
    }

    pub async fn get_latest_commit(
        &self,
        repo_owner: String,
        repo_name: String,
        token: String,
        branch: Option<String>,
    ) -> Result<String, Error> {
        let url = format!("{}/{}/{}/commits", GITHUB_API_URL, repo_owner, repo_name);

        let url = if let Some(branch) = branch {
            format!("{}?sha={}", url, branch)
        } else {
            url
        };

        let commits = self
            .client
            .get::<Vec<serde_json::Value>>(url, token)
            .await?;
        let latest_commit = commits
            .get(0)
            .and_then(|commit| commit["sha"].as_str())
            .ok_or(Error::NoCommitFound)?;

        Ok(latest_commit.to_string())
    }

    pub async fn get_pull_requests(
        &self,
        repo_owner: String,
        repo_name: String,
        token: String,
    ) -> Result<Vec<PullRequest>, Error> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/pulls",
            repo_owner, repo_name
        );

        let pull_requests: Vec<PullRequest> = self.client.get(url, token).await?;
        Ok(pull_requests)
    }
}
