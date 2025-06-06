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
    pub fn new(global_config: GlobalConfig, agent: AgentApp, controller: ControllerApp, monitor: MonitorApp) -> Self {
        Self {
            global_config: Arc::new(RwLock::new(global_config.clone())),
            agent: SealedService::new(agent, global_config.clone()),
            controller: SealedService::new(controller,
                global_config.clone()),
            monitor: SealedService::new(monitor, global_config),
        }
    }

    pub async fn toggle_service(&mut self, toggle_service: Services) -> Result<(), Error> {
        match toggle_service {
            Services::Agent(toggle) => {
                if toggle {
                    self.agent.enable().await.map_err(Error::ToggleAgentError)?;
                } else {
                    self.agent
                        .disable()
                        .await
                        .map_err(Error::ToggleAgentError)?;
                }
            }
            Services::ReleaseAgent(toggle) => {
                // Placeholder for release_agent toggle logic
            }
            Services::Scheduler(toggle) => {
                // Placeholder for scheduler toggle logic
            }
            Services::Monitor(toggle) => {
                // Placeholder for monitor toggle logic
            }
            Services::Controller(toggle) => {
                if toggle {
                    self.controller
                        .enable()
                        .await
                        .map_err(Error::ToggleControllerError)?;
                } else {
                    self.controller
                        .disable()
                        .await
                        .map_err(Error::ToggleControllerError)?;
                }
            }
        }

        Ok(())
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
