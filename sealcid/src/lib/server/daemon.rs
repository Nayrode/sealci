use std::sync::Arc;

use agent::{app::App as AgentApp, config::Config as AgentConfig};
use tokio::sync::RwLock;

use crate::{
    common::{
        error::Error,
        mutation::{
            AgentMutation, Apply, ControllerMutation, MonitorMutation, ReleaseAgentMutation,
            SchedulerMutation,
        },
        service_enum::Services,
    },
    server::{config::GlobalConfig, service::SealedService},
};

pub struct Daemon {
    pub global_config: Arc<RwLock<GlobalConfig>>,
    pub agent: SealedService<AgentApp, AgentConfig>,
}

impl Daemon {
    pub fn new(global_config: GlobalConfig, agent: AgentApp) -> Self {
        Self {
            global_config: Arc::new(RwLock::new(global_config.clone())),
            agent: SealedService::new(agent, global_config.clone()),
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
                // Placeholder for controller toggle logic
            }
        }

        Ok(())
    }

    pub async fn mutate_agent(&mut self, config: AgentMutation) -> Result<(), Error> {
        let global_config = self.global_config.read().await;
        let mut agent_config: AgentConfig = global_config.to_owned().into();
        let config = config.apply(&mut agent_config);
        self.agent
            .restart_with_config(config)
            .await
            .map_err(Error::RestartAgentError)?;
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

    pub async fn mutate_scheduler(&mut self, config: SchedulerMutation) -> Result<(), Error> {
        // Placeholder for scheduler mutation logic
        // Restart agent, scheduler
        Ok(())
    }

    pub async fn mutate_monitor(&mut self, config: MonitorMutation) -> Result<(), Error> {
        // Placeholder for monitor mutation logic
        // Restart  monitor
        Ok(())
    }

    pub async fn mutate_controller(&mut self, config: ControllerMutation) -> Result<(), Error> {
        // Placeholder for monitor mutation logic
        // Restart controller, monitor
        Ok(())
    }
}
