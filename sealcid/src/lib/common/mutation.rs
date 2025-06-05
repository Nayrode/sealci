pub trait Apply<Config> {
    /// Applies the mutation to the given configuration.
    fn apply(self, config: &mut Config) -> Config;
}

pub struct AgentMutation {
    pub enable_agent: bool,
    // Example: http://hugo.fr
    pub agent_host: Option<String>,
    // Example: 8080
    pub agent_port: Option<u32>,
}

impl Apply<agent::config::Config> for AgentMutation {
    fn apply(self, config: &mut agent::config::Config) -> agent::config::Config {
        if let Some(host) = self.agent_host {
            config.ahost = host;
        }
        if let Some(port) = self.agent_port {
            config.port = port;
        }
        config.to_owned()
    }
}

pub struct SchedulerMutation {
    pub enable_agent: bool,
    // Example: http://hugo.fr
    pub scheduler_host: Option<String>,
    // Example: 8080
    pub scheduler_port: Option<String>,
}

pub struct MonitorMutation {
    pub monitor_port: Option<String>,
}

pub struct ControllerMutation {
    pub enable_agent: bool,
    // Example: http://hugo.fr
    pub controller_host: Option<String>,
    // Example: 8080
    pub controller_port: Option<String>,
    // Postgres url for the controller
    pub database_url: Option<String>,
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
