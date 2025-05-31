use crate::common::GitEvent;
use crate::github::GitHubClient;
use crate::{controller::ControllerClient, error::Error};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::sync::{Arc, RwLock};
use tokio::sync::Mutex;
use tokio::task::JoinSet;
use tokio::time::{sleep, Duration};
use tracing::{error, info};

#[derive(serde::Serialize)]
pub struct Listener {
    repo_owner: String,
    repo_name: String,
    repo_url: String,
    events: RwLock<Vec<GitEvent>>,
    #[serde(skip)]
    actions_file: RwLock<Arc<File>>,
    #[serde(skip)]
    github_client: Arc<GitHubClient>,
    #[serde(skip)]
    controller_client: Arc<ControllerClient>,
    #[serde(skip)]
    listener_handles: Mutex<JoinSet<()>>,
    #[serde(skip)]
    github_token: String,
}

impl Listener {
    pub fn new(
        repo_owner: String,
        repo_name: String,
        events: Vec<GitEvent>,
        actions_file: Arc<File>,
        github_client: Arc<GitHubClient>,
        controller_client: Arc<ControllerClient>,
        github_token: String,
    ) -> Self {
        let repo_url = format!("https://github.com/{}/{}", repo_owner, repo_name);

        let events = RwLock::new(events);
        let actions_file = RwLock::new(actions_file);
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
            github_token,
        }
    }

    pub async fn listen_to_commits(&self) -> Result<(), Error> {
        info!("Searching commits....");

        let last_commit = self
            .github_client
            .get_latest_commit(
                self.repo_owner.clone(),
                self.repo_name.clone(),
                self.github_token.clone(),
                None,
            )
            .await?;

        info!("Last commit found: {}", last_commit);

        let repo_owner = self.repo_owner.clone();
        let repo_name = self.repo_name.clone();
        let github_token = self.github_token.clone();
        let repo_url = self.repo_url.clone();
        let file = self.actions_file.read().unwrap().clone();
        let github_client = self.github_client.clone();
        let controller_client = self.controller_client.clone();
        let mut listener_handles = self.listener_handles.lock().await;
        listener_handles.spawn(async move {
            let mut last_commit = last_commit;
            loop {
                sleep(Duration::from_secs(10)).await; // Wait 10 seconds before checking again

                match github_client
                    .get_latest_commit(
                        repo_owner.clone(),
                        repo_name.clone(),
                        github_token.clone(),
                        None,
                    )
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
            .get_pull_requests(
                self.repo_owner.clone(),
                self.repo_name.clone(),
                self.github_token.clone(),
            )
            .await?;
        let last_pr = last_pull_requests
            .get(0)
            .ok_or(Error::NoPullRequestFound)?
            .to_owned();

        let repo_owner = self.repo_owner.clone();
        let repo_name = self.repo_name.clone();
        let github_token = self.github_token.clone();
        let repo_url = self.repo_url.clone();
        let file = self.actions_file.read().unwrap().clone();
        let github_client = self.github_client.clone();
        let controller_client = self.controller_client.clone();
        let mut listener_handles = self.listener_handles.lock().await;

        listener_handles.spawn(async move {
            let mut last_pr = last_pr;
            loop {
                sleep(Duration::from_secs(10)).await; // Wait 10 seconds before checking again

                match github_client
                    .get_pull_requests(repo_owner.clone(), repo_name.clone(), github_token.clone())
                    .await
                {
                    Ok(current_tags) => {
                        if let Some(last_tag_pushed) = current_tags.get(0) {
                            if last_pr.id != last_tag_pushed.id {
                                info!(
                                    "{}/{} - New pull request found: {}",
                                    repo_owner, repo_name, last_tag_pushed.title
                                );
                                last_pr = last_tag_pushed.clone(); // Update the last PR ID
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

    pub async fn listen_to_tags(&self) -> Result<(), Error> {
        let tags = self
            .github_client
            .get_tags(
                self.repo_owner.clone(),
                self.repo_name.clone(),
                self.github_token.clone(),
            )
            .await?;
        // If no tags are found, we initialize with an empty list
        let last_tags = tags;

        let repo_owner = self.repo_owner.clone();
        let repo_name = self.repo_name.clone();
        let github_token = self.github_token.clone();
        let repo_url = self.repo_url.clone();
        let file = self.actions_file.read().unwrap().clone();
        let github_client = self.github_client.clone();
        let controller_client = self.controller_client.clone();
        let mut listener_handles = self.listener_handles.lock().await;

        listener_handles.spawn(async move {
            let mut last_tags = last_tags;
            loop {
                sleep(Duration::from_secs(10)).await; // Wait 10 seconds before checking again

                match github_client
                    .get_tags(repo_owner.clone(), repo_name.clone(), github_token.clone())
                    .await
                {
                    Ok(current_tags) => {
                        info!("Current tags: {:?}", current_tags);

                        if current_tags != last_tags {
                            info!(
                                "{}/{} - New tags detected: {:?}",
                                repo_owner, repo_name, current_tags
                            );
                            if let Err(e) = controller_client
                                .send_to_controller(&repo_url, file.as_ref())
                                .await
                            {
                                error!("Error sending to controller: {}", e);
                                continue;
                            }
                            last_tags = current_tags; // Update the last tags
                        }
                    }
                    Err(_) => {
                        // Handle errors (such as network issues or API problems)
                        error!("Error fetching the latest tags");
                    }
                };
            }
        });
        Ok(())
    }

    pub async fn listen_to_all(&self) -> Result<(), Error> {
        self.listen_to_commits().await?;
        self.listen_to_pull_requests().await?;
        self.listen_to_tags().await?;
        Ok(())
    }

    pub async fn start(&self) -> Result<(), Error> {
        // Check if the listener should listen to all events
        // If it does, we set it to listen to all events
        // We prevent adding the same listener multiple times
        let events = self.events.read().unwrap().clone();
        if events.contains(&GitEvent::All) {
            self.listen_to_all().await?;
        } else {
            for e in &events {
                match e {
                    GitEvent::Commit => self.listen_to_commits().await?,
                    GitEvent::PullRequest => self.listen_to_pull_requests().await?,
                    GitEvent::Tag => self.listen_to_tags().await?,

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

    pub async fn update(
        &self,
        events: Option<Vec<GitEvent>>,
        actions_file: Option<File>,
    ) -> Result<(), Error> {
        self.stop().await; // Stop the listener before updating
        if let Some(new_events) = events {
            *self.events.write().unwrap() = new_events;
        }
        if let Some(new_actions_file) = actions_file {
            let new_actions_file = Arc::new(new_actions_file);
            *self.actions_file.write().unwrap() = new_actions_file;
        }
        self.start().await?;
        Ok(())
    }

    pub async fn action_file_to_string(&self) -> Result<String, Error> {
        let file = self.actions_file.read().unwrap();
        let mut content = String::new();

        let mut file_ref = file.as_ref();
        if let Err(_) = file_ref.seek(SeekFrom::Start(0)) {
            return Err(Error::Error(
                "Failed to seek to the start of the file".to_string(),
            ));
        }
        file_ref
            .read_to_string(&mut content)
            .map_err(Error::FileReadError)?;
        Ok(content)
    }
}
