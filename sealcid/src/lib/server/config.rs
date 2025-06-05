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

impl Into<agent::config::Config> for GlobalConfig {
    fn into(self) -> agent::config::Config {
        agent::config::Config {
            shost: self.scheduler_host + ":" + &self.scheduler_port,
            ahost: self.agent_host,
            port: self.agent_port,
        }
    }
}

impl Into<controller::config::Config> for GlobalConfig {
    fn into(self) -> controller::config::Config {
        controller::config::Config {
            http: self.controller_host + ":" + &self.controller_port,
            database_url: self.database_url,
            grpc: self.scheduler_host + ":" + &self.scheduler_port,
        }
    }
}
