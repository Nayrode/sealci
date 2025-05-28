use crate::controller::ControllerClient;
use crate::github::GitHubClient;
use std::error::Error;
use tokio::time::{sleep, Duration};
use tracing::{error, info};

use std::path::Path;

pub struct Listener {
    pub repo_owner: String,
    pub repo_name: String,
    pub repo_url: String,
    pub event: String,
    pub actions_path: Box<Path>,
    pub github_client: GitHubClient,
    pub controller_client: ControllerClient,
}

impl Listener {
    pub fn new(
        repo_owner: String,
        repo_name: String,
        event: String,
        actions_path: Box<Path>,
        github_client: GitHubClient,
        controller_client: ControllerClient,
    ) -> Self {
        let repo_url = format!("https://github.com/{}/{}", repo_owner, repo_name);

        Listener {
            repo_owner,
            repo_name,
            repo_url,
            event,
            actions_path,
            github_client,
            controller_client,
        }
    }

    pub async fn listen_to_commits(&self) -> Result<(), Box<dyn Error>> {
        // Get the latest commit and unwrap the result properly
        let mut last_commit = self
            .github_client
            .get_latest_commit(&self.repo_owner, &self.repo_name, None)
            .await?;
        info!("Last commit found: {}", last_commit);

        loop {
            sleep(Duration::from_secs(10)).await; // Wait 10 seconds before checking again
            info!(
                "{}/{} - Checking for new commits...",
                self.repo_owner, self.repo_name
            );

            // Handle the Result from `get_latest_commit`
            match self
                .github_client
                .get_latest_commit(&self.repo_owner, &self.repo_name, None)
                .await
            {
                Ok(current_commit) => {
                    // Compare the latest commit with the current one
                    if last_commit != current_commit {
                        info!(
                            "{}/{} - New commit found: {}",
                            self.repo_owner, self.repo_name, current_commit
                        );
                        last_commit = current_commit;

                        self.controller_client
                            .send_to_controller(&self.repo_url, &self.actions_path)
                            .await?;
                    }
                }
                Err(e) => {
                    // Handle errors (such as network issues or API problems)
                    error!("Error fetching the latest commit: {}", e);
                }
            }
        }
    }

    pub async fn listen_to_pull_requests(&self) -> Result<(), Box<dyn Error>> {
        let last_pull_requests = self
            .github_client
            .get_pull_requests(&self.repo_owner, &self.repo_name)
            .await?;
        let mut last_pr = last_pull_requests
            .get(0)
            .ok_or_else(|| "No pull requests found")?
            .to_owned();
        info!(
            "{}/{} - Found pull request: {}",
            self.repo_owner, self.repo_name, last_pr.title
        );

        loop {
            sleep(Duration::from_secs(10)).await; // Wait 10 seconds before checking again
            info!(
                "{}/{} - Checking for new pull requests...",
                self.repo_owner, self.repo_name
            );

            match self
                .github_client
                .get_pull_requests(&self.repo_owner, &self.repo_name)
                .await
            {
                Ok(current_pull_requests) => {
                    if let Some(current_pr) = current_pull_requests.get(0) {
                        if last_pr.id != current_pr.id {
                            info!(
                                "{}/{} - New pull request found: {}",
                                self.repo_owner, self.repo_name, current_pr.title
                            );
                            last_pr = current_pr.clone(); // Update the last PR ID
                            self.controller_client
                                .send_to_controller(&self.repo_url, &self.actions_path)
                                .await?;
                        }
                    }
                }
                Err(e) => {
                    // Handle errors (such as network issues or API problems)
                    error!("Error fetching the latest pull request: {}", e);
                }
            }
        }
    }
}
