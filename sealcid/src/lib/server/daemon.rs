use std::sync::Arc;

use agent::{app::App as AgentApp, config::Config as AgentConfig};
use controller::{application::App as ControllerApp, config::Config as ControllerConfig};
use monitor::{app::App as MonitorApp, config::Config as MonitorConfig};
use tokio::sync::RwLock;

use crate::{
    common::{
        error::Error,
        mutation::{
            ControllerMutation, ReleaseAgentMutation, SchedulerMutation,
        },
        service_enum::Services,
    },
    server::{config::GlobalConfig, service::SealedService},
};

pub struct Daemon {
    pub global_config: Arc<RwLock<GlobalConfig>>,
    pub agent: SealedService<AgentApp, AgentConfig>,
    pub controller: SealedService<ControllerApp, ControllerConfig>,
    pub monitor: SealedService<MonitorApp, MonitorConfig>,
}

impl Daemon {
    pub fn new(
        global_config: GlobalConfig,
        agent: AgentApp,
        controller: ControllerApp,
        monitor: MonitorApp,
    ) -> Self {
        Self {
            global_config: Arc::new(RwLock::new(global_config.clone())),
            agent: SealedService::new(agent, global_config.clone()),
            controller: SealedService::new(controller, global_config.clone()),
            monitor: SealedService::new(monitor, global_config),
        }
    }

    pub async fn mutate_release_agent(
        &mut self,
        config: ReleaseAgentMutation,
    ) -> Result<(), Error> {
        // Placeholder for release_agent mutation logic
        // Restart controller, release agent
        Ok(())
    }
}
