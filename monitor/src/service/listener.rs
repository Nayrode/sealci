use std::{collections::HashMap, fs::File, sync::Arc};

use tokio::sync::RwLock;

use crate::{
    common::GitEvent, controller::ControllerClient, error::Error, event_listener::Listener,
    github::GitHubClient,
};

pub struct ListenerService {
    listeners: Arc<RwLock<HashMap<String, Arc<Listener>>>>,
    github_client: Arc<GitHubClient>,
    controller_client: Arc<ControllerClient>,
}

impl ListenerService {
    pub fn new(github_client: Arc<GitHubClient>, controller_client: Arc<ControllerClient>) -> Self {
        ListenerService {
            listeners: Arc::new(RwLock::new(HashMap::new())),
            github_client,
            controller_client,
        }
    }

    pub async fn add_listener(
        &self,
        repo_owner: String,
        repo_name: String,
        actions_file: Arc<File>,
        on_events: Vec<GitEvent>,
        github_token: String,
    ) -> Result<Arc<Listener>, Error> {
        let listener = Listener::new(
            repo_owner.clone(),
            repo_name.clone(),
            on_events,
            actions_file,
            self.github_client.clone(),
            self.controller_client.clone(),
            github_token,
        );

        listener.start().await?;

        let mut listeners = self.listeners.write().await;
        let listener = Arc::new(listener);
        let key = format!("{}-{}", repo_owner, repo_name);
        listeners.insert(key, listener.clone());
        Ok(listener)
    }

    pub async fn remove_listener(&self, key: String) -> Result<Arc<Listener>, Error> {
        let mut listeners = self.listeners.write().await;
        let listener = listeners
            .get(key.as_str())
            .ok_or(Error::NoListenerFound)?
            .clone();
        // Stop the listener if it exists
        listener.stop().await;
        let listener = listeners.remove(key.as_str());
        let listener = listener.ok_or(Error::NoListenerFound)?;
        Ok(listener)
    }

    pub async fn update_listener(
        &self,
        key: String,
        on_events: Option<Vec<GitEvent>>,
        actions_file: Option<File>,
    ) -> Result<Arc<Listener>, Error> {
        let mut listeners = self.listeners.write().await;
        let listener = listeners
            .get_mut(key.as_str())
            .ok_or(Error::NoListenerFound)?;
        // Update the events and restart the listener
        listener.update(on_events, actions_file).await?;
        Ok(listener.clone())
    }

    pub async fn get_listener(&self, key: String) -> Result<Arc<Listener>, Error> {
        let listeners = self.listeners.read().await;
        Ok(listeners
            .get(key.as_str())
            .ok_or(Error::NoListenerFound)?
            .to_owned())
    }

    pub async fn get_all_listeners(&self) -> Result<Vec<Arc<Listener>>, Error> {
        let listeners = self.listeners.read().await;
        Ok(listeners.values().cloned().collect())
    }
}
