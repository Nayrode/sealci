use crate::error;
use crate::{controller::ControllerClient, error::Error};
use crate::github::GitHubClient;
use std::sync::Arc;
use tokio::{
    spawn,
    task::JoinHandle,
    time::{sleep, Duration},
};
use tracing::{error, info};

use std::path::Path;

pub struct Listener {
    pub repo_owner: String,
    pub repo_name: String,
    pub repo_url: String,
    pub event: String,
    pub actions_path: Box<Path>,
    pub github_client: Arc<GitHubClient>,
    pub controller_client: Arc<ControllerClient>,
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

        // Wrap the GitHubClient and ControllerClient in Arc for shared ownership
        let github_client = Arc::new(github_client);
        let controller_client = Arc::new(controller_client);
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

    pub async fn listen_to_commits(&self) -> Result<JoinHandle<()>, Error> {
        let last_commit = match self
            .github_client
            .get_latest_commit(&self.repo_owner, &self.repo_name, None)
            .await
        {
            Ok(val) => val,
            Err(e) => {
                error!("Error fetching the latest commit: {}", e);
                return Err(error::Error::Error);
            }
        };
        info!("Last commit found: {}", last_commit);

        let repo_owner = self.repo_owner.clone();
        let repo_name = self.repo_name.clone();
        let repo_url = self.repo_url.clone();
        let actions_path = self.actions_path.clone();
        let github_client = self.github_client.clone();
        let controller_client = self.controller_client.clone();

        let handle = spawn(async move {
            let mut last_commit = last_commit;
            loop {
                sleep(Duration::from_secs(10)).await; // Wait 10 seconds before checking again
                info!("{}/{} - Checking for new commits...", repo_owner, repo_name);

                match github_client
                    .get_latest_commit(&repo_owner, &repo_name, None)
                    .await
                {
                    Ok(current_commit) => {
                        if last_commit != current_commit {
                            info!(
                                "{}/{} - New commit found: {}",
                                repo_owner, repo_name, current_commit
                            );
                            last_commit = current_commit;

                            if let Err(e) = controller_client
                                .send_to_controller(&repo_url, &actions_path)
                                .await
                            {
                                error!("Error sending to controller: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        error!("Error fetching the latest commit: {}", e);
                    }
                }
            }
        });
        Ok(handle)
    }

    pub async fn listen_to_pull_requests(
        &self,
    ) -> Result<JoinHandle<()>, Box<dyn Error + Send + Sync>> {
        let last_pull_requests = self
            .github_client
            .get_pull_requests(&self.repo_owner, &self.repo_name)
            .await?;
        let last_pr = last_pull_requests
            .get(0)
            .ok_or_else(|| "No pull requests found".to_string())?
            .to_owned();
        info!(
            "{}/{} - Found pull request: {}",
            self.repo_owner, self.repo_name, last_pr.title
        );

        let repo_owner = self.repo_owner.clone();
        let repo_name = self.repo_name.clone();
        let repo_url = self.repo_url.clone();
        let actions_path = self.actions_path.clone();
        let github_client = self.github_client.clone();
        let controller_client = self.controller_client.clone();

        let handle = spawn(async move {
            let mut last_pr = last_pr;
            loop {
                sleep(Duration::from_secs(10)).await; // Wait 10 seconds before checking again
                info!(
                    "{}/{} - Checking for new pull requests...",
                    repo_owner, repo_name
                );

                match github_client
                    .get_pull_requests(&repo_owner, &repo_name)
                    .await
                {
                    Ok(current_pull_requests) => {
                        if let Some(current_pr) = current_pull_requests.get(0) {
                            if last_pr.id != current_pr.id {
                                info!(
                                    "{}/{} - New pull request found: {}",
                                    repo_owner, repo_name, current_pr.title
                                );
                                last_pr = current_pr.clone(); // Update the last PR ID
                                if let Err(e) = controller_client
                                    .send_to_controller(&repo_url, &actions_path)
                                    .await
                                {
                                    error!("Error sending to controller: {}", e);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        // Handle errors (such as network issues or API problems)
                        error!("Error fetching the latest pull request: {}", e);
                    }
                };
            }
        });
        Ok(handle)
    }
}
