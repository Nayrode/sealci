use crate::common::proto::{AgentMutation, ControllerMutation, MonitorMutation, SchedulerMutation};

pub trait Update<Mutation> {
    /// Updates the configuration with the given mutation.
    fn update(&mut self, mutation: Mutation);
}

#[derive(Debug, Clone)]
pub struct GlobalConfig {
    // Example: 8080
    pub monitor_port: String,

    // Example: http://hugo.fr
    pub controller_host: String,
    // Example: 8080
    pub controller_port: String,
    // Postgres url for the controller
    pub database_url: String,

    // Example: http://hugo.fr
    pub release_agent_host: String,
    // Example: 8080
    pub release_agent_port: String,

    // Other configuration for the release agent
    pub passphrase: String,
    pub secret_key: String,
    pub git_path: String,
    pub bucket_addr: String,
    pub bucket_access_key: String,
    pub bucket_secret_key: String,
    pub bucket_name: String,

    // Example: http://hugo.fr
    pub scheduler_host: String,
    // Example: 8080
    pub scheduler_port: String,

    // Example: http://hugo.fr
    pub agent_host: String,
    // Example: 8080
    pub agent_port: u32,
}

impl Update<AgentMutation> for GlobalConfig {
    fn update(&mut self, mutation: AgentMutation) {
        if let Some(ahost) = mutation.agent_host {
            self.agent_host = ahost;
        }
        if let Some(port) = mutation.agent_port {
            self.agent_port = port;
        }
    }
}

impl Update<ControllerMutation> for GlobalConfig {
    fn update(&mut self, mutation: ControllerMutation) {
        if let Some(host) = mutation.controller_host {
            self.controller_host = host;
        }
        if let Some(port) = mutation.controller_port {
            self.controller_port = port;
        }
        if let Some(database_url) = mutation.database_url {
            self.database_url = database_url;
        }
    }
}

impl Update<MonitorMutation> for GlobalConfig {
    fn update(&mut self, mutation: MonitorMutation) {
        if let Some(port) = mutation.monitor_port {
            self.monitor_port = port.to_string();
        }
        // if let Some(controller_host) = mutation. {
        //     self.controller_host = controller_host;
        // }
    }
}

impl Update<SchedulerMutation> for GlobalConfig {
    fn update(&mut self, mutation: SchedulerMutation) {
        if let Some(host) = mutation.scheduler_host {
            self.scheduler_host = host;
        }
        if let Some(port) = mutation.scheduler_port {
            self.scheduler_port = port;
        }
    }
}

impl Into<agent::config::Config> for GlobalConfig {
    fn into(self) -> agent::config::Config {
        agent::config::Config {
            shost: self.scheduler_host + ":" + &self.scheduler_port,
            ahost: "0.0.0.0".to_string(),
            port: self.agent_port,
        }
    }
}

impl Into<controller::config::Config> for GlobalConfig {
    fn into(self) -> controller::config::Config {
        controller::config::Config {
            http: format!("0.0.0.0:{}", self.controller_port),
            database_url: self.database_url,
            grpc: self.scheduler_host + ":" + &self.scheduler_port,
        }
    }
}

impl Into<monitor::config::Config> for GlobalConfig {
    fn into(self) -> monitor::config::Config {
        monitor::config::Config {
            controller_host: "0.0.0.0".to_string(),
            port: self.monitor_port.parse().unwrap_or(9001),
        }
    }
}

impl Into<sealci_scheduler::app::Config> for GlobalConfig {
    fn into(self) -> sealci_scheduler::app::Config {
        sealci_scheduler::app::Config {
            addr: format!("0.0.0.0:{}", self.scheduler_port),
        }
    }
}
