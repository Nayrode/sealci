use crate::common::GitEvent;
use crate::github::GitHubClient;
use crate::{controller::ControllerClient, error::Error};
use std::fs::File;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::JoinSet;
use tokio::time::{sleep, Duration};
use tracing::{error, info};

#[derive(serde::Serialize)]
pub struct Listener {
    pub repo_owner: String,
    pub repo_name: String,
    pub repo_url: String,
    pub events: Vec<GitEvent>,
    #[serde(skip)]
    pub actions_file: Arc<File>,
    #[serde(skip)]
    pub github_client: Arc<GitHubClient>,
    #[serde(skip)]
    pub controller_client: Arc<ControllerClient>,
    #[serde(skip)]
    listener_handles: Mutex<JoinSet<()>>,
}

impl Listener {
    pub fn new(
        repo_owner: String,
        repo_name: String,
        events: Vec<GitEvent>,
        actions_file: File,
        github_client: Arc<GitHubClient>,
        controller_client: Arc<ControllerClient>,
    ) -> Self {
        let repo_url = format!("https://github.com/{}/{}", repo_owner, repo_name);
        let actions_file = Arc::new(actions_file);
        
        // Wrap the GitHubClient and ControllerClient in Arc for shared ownership
        let listerner_handles = Mutex::new(JoinSet::new());
        Listener {
            repo_owner,
            repo_name,
            repo_url,
            events,
            actions_file,
            github_client,
            controller_client,
            listener_handles: listerner_handles,
        }
    }

    pub async fn listen_to_commits(&self) -> Result<(), Error> {
        let last_commit = self
            .github_client
            .get_latest_commit(&self.repo_owner, &self.repo_name, None)
            .await?;

        info!("Last commit found: {}", last_commit);

        let repo_owner = self.repo_owner.clone();
        let repo_name = self.repo_name.clone();
        let repo_url = self.repo_url.clone();
        let file = self.actions_file.clone();
        let github_client = self.github_client.clone();
        let controller_client = self.controller_client.clone();
        let mut listener_handles = self.listener_handles.lock().await;
        listener_handles.spawn(async move {
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
                                .send_to_controller(&repo_url, file.as_ref())
                                .await
                            {
                                error!("Error sending to controller: {}", e);
                            }
                        }
                    }
                    Err(_) => {
                        error!("Error fetching the latest commit",);
                    }
                }
            }
        });

        Ok(())
    }

    pub async fn listen_to_pull_requests(&self) -> Result<(), Error> {
        let last_pull_requests = self
            .github_client
            .get_pull_requests(&self.repo_owner, &self.repo_name)
            .await?;
        let last_pr = last_pull_requests
            .get(0)
            .ok_or(Error::NoPullRequestFound)?
            .to_owned();

        let repo_owner = self.repo_owner.clone();
        let repo_name = self.repo_name.clone();
        let repo_url = self.repo_url.clone();
        let file = self.actions_file.clone();
        let github_client = self.github_client.clone();
        let controller_client = self.controller_client.clone();
        let mut listener_handles = self.listener_handles.lock().await;

        listener_handles.spawn(async move {
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
                                    .send_to_controller(&repo_url, file.as_ref())
                                    .await
                                {
                                    error!("Error sending to controller: {}", e);
                                }
                            }
                        }
                    }
                    Err(_) => {
                        // Handle errors (such as network issues or API problems)
                        error!("Error fetching the latest pull request");
                    }
                };
            }
        });
        Ok(())
    }

    pub async fn listen_to_all(&mut self) -> Result<(), Error> {
        self.listen_to_commits().await?;
        self.listen_to_pull_requests().await?;
        Ok(())
    }

    pub async fn start(&mut self) -> Result<(), Error> {
        // Check if the listener should listen to all events
        // If it does, we set it to listen to all events
        // We prevent adding the same listener multiple times

        if self.events.contains(&GitEvent::All) {
            self.listen_to_all().await?;
        } else {
            for e in &self.events {
                match e {
                    GitEvent::Commit => self.listen_to_commits().await?,
                    GitEvent::PullRequest => self.listen_to_pull_requests().await?,
                    GitEvent::All => (),
                }
            }
        }
        Ok(())
    }

    pub async fn stop(&self) {
        // Cancel all listener tasks
        let mut listener_handles = self.listener_handles.lock().await;
        listener_handles.abort_all();
    }
}
