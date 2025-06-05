use agent::config::Config as AgentConfig;
use controller::config::Config as ControllerConfig;
use monitor::config::Config as MonitorConfig;

pub(crate) use crate::common::proto::{AgentMutation, ControllerMutation, MonitorMutation};

pub trait Apply<Config> {
    /// Applies the mutation to the given configuration.
    fn apply(&self, config: &mut Config);
}

impl Apply<AgentConfig> for AgentMutation {
    fn apply(&self, config: &mut AgentConfig) {
        if let Some(ahost) = self.agent_host.clone().take() {
            config.ahost = ahost.to_owned();
        }
        if let Some(port) = self.agent_port.clone().take() {
            config.port = port;
        }
    }
}

pub struct SchedulerMutation {
    pub enable_agent: bool,
    // Example: http://hugo.fr
    pub scheduler_host: Option<String>,
    // Example: 8080
    pub scheduler_port: Option<String>,
}

impl Apply<MonitorConfig> for MonitorMutation {
    fn apply(&self, config: &mut MonitorConfig) {
        if let Some(port) = self.monitor_port.clone() {
            config.port = 2;
        }
    }
}

impl Apply<ControllerConfig> for ControllerMutation {
    fn apply(&self, config: &mut ControllerConfig) {
        if let Some(host) = self.controller_host.clone() {
            config.http = host;
        }
        if let Some(port) = self.controller_port.clone() {
            config.http = format!("{}:{}", config.http, port);
        }
        if let Some(db_url) = self.database_url.clone() {
            config.database_url = db_url;
        }
    }
}

pub struct ReleaseAgentMutation {
    pub enable_agent: bool,

    pub release_agent_host: Option<String>,
    // Example: 8080
    pub release_agent_port: Option<String>,

    // Other configuration for the release agent
    pub passphrase: Option<String>,
    pub secret_key: Option<String>,
    pub git_path: Option<String>,
    pub bucket_addr: Option<String>,
    pub bucket_access_key: Option<String>,
    pub bucket_secret_key: Option<String>,
    pub bucket_name: Option<String>,
}
