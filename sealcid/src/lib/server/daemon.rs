use std::sync::Arc;

use agent::{app::App, config::Config};
use tokio::sync::RwLock;

use crate::{
    common::error::Error,
    server::{config::GlobalConfig, service::SealedService},
};

pub struct Daemon {
    pub global_config: Arc<RwLock<GlobalConfig>>,
    pub agent: SealedService<App, Config>,
}

impl Daemon {
    pub fn new(global_config: GlobalConfig, agent: App) -> Self {
        Self {
            global_config: Arc::new(RwLock::new(global_config.clone())),
            agent: SealedService::new(agent, global_config.clone()),
        }
    }

    pub async fn mutate_agent(&mut self) -> Result<(), Error> {
        self.agent.restart().await.map_err(Error::RestartAgentError)?;
        Ok(())
    }
}
