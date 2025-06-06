use std::sync::Arc;

use agent::{app::App as AgentApp, config::Config as AgentConfig};
use controller::{application::App as ControllerApp, config::Config as ControllerConfig};
use monitor::{app::App as MonitorApp, config::Config as MonitorConfig};
use sealci_scheduler::app::{App as SchedulerApp, Config as SchedulerConfig};
use tokio::sync::RwLock;

use crate::server::{config::GlobalConfig, service::SealedService};

pub struct Daemon {
    pub global_config: Arc<RwLock<GlobalConfig>>,
    pub agent: SealedService<AgentApp, AgentConfig>,
    pub controller: SealedService<ControllerApp, ControllerConfig>,
    pub monitor: SealedService<MonitorApp, MonitorConfig>,
    pub scheduler: SealedService<SchedulerApp, SchedulerConfig>,
}

impl Daemon {
    pub fn new(
        global_config: GlobalConfig,
        agent: AgentApp,
        controller: ControllerApp,
        monitor: MonitorApp,
        scheduler: SchedulerApp,
    ) -> Self {
        Self {
            global_config: Arc::new(RwLock::new(global_config.clone())),
            agent: SealedService::new(agent, global_config.clone()),
            controller: SealedService::new(controller, global_config.clone()),
            monitor: SealedService::new(monitor, global_config.clone()),
            scheduler: SealedService::new(scheduler, global_config.clone()),
        }
    }
}
