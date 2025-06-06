pub(crate) use crate::common::proto::{AgentMutation, ControllerMutation, MonitorMutation};

pub trait Apply<Config> {
    /// Applies the mutation to the given configuration.
    fn apply(&mut self, config: Config);
}

impl Apply<agent::config::Config> for AgentMutation {
    fn apply(&mut self, config: &mut agent::config::Config) {
        if let Some(ahost) = self.agent_host.take() {
            config.ahost = ahost;
        }
        if let Some(port) = self.agent_port.take() {
            config.port = port;
        }
    }
}

impl Apply<monitor::config::Config> for MonitorMutation {
    fn apply(self, config: &mut monitor::config::Config) {
        if let Some(port) = self.monitor_port {
            config.port = port.parse().unwrap();
        }
    }
}

impl Apply<controller::config::Config> for ControllerMutation {
    fn apply(self, config: &mut controller::config::Config) {
        if let Some(host) = self.controller_host {
            config.http = host;
        }
        if let Some(port) = self.controller_port {
            config.http = format!("0.0.0.0:{}", config.http, port);
        }
        if let Some(db_url) = self.database_url {
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
