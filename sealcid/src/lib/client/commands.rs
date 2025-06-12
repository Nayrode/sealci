use std::{fs::File, io::Read, path::PathBuf};
use std::fmt::{Display, Formatter};
use crate::{
    client::{config::ClientConfig, error::ClientError},
    common::proto::{
        AgentMutation, ControllerMutation, MonitorMutation, ReleaseAgentMutation,
        SchedulerMutation, daemon_client::DaemonClient, Services, ServiceStatus,
        StatusRequest, StatusResponse,
    },
};
use clap::{Parser, Subcommand};
use tonic::{Request, transport::Channel};

const SEAL_CONFIG_DEFAULT: &str = ".seal/config";

#[derive(Debug, Clone, Parser)]
#[command(name = "app")]
pub struct Cli {
    #[arg(long, default_value = SEAL_CONFIG_DEFAULT, env = "SEALCONFIG")]
    sealconfig: PathBuf,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Clone, Copy, Subcommand)]
pub enum Toggle {
    Status,
    Start,
    Stop,
}

impl Toggle {
    pub fn as_bool(&self) -> bool {
        self == &Toggle::Start
    }
}

impl PartialEq for Toggle {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Toggle::Start, Toggle::Start) | (Toggle::Stop, Toggle::Stop)
        )
    }
}

impl Display for StatusResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for status in &self.statuses {
            let service_name = match status.service() {
                Services::Agent => "Agent",
                Services::Scheduler => "Scheduler",
                Services::Monitor => "Monitor",
                Services::Controller => "Controller",
                Services::ReleaseAgent => "Release Agent",
            };

            let status_str = match status.status() {
                ServiceStatus::Running => "RUNNING",
                ServiceStatus::Stopped => "STOPPED",
                ServiceStatus::Booting => "BOOTING",
                ServiceStatus::Error => "ERROR",
            };

            writeln!(f, "{}: {}", service_name, status_str)?;
        }
        Ok(())
    }
}


#[derive(Debug, Clone, Subcommand)]
pub enum Commands {
    /// Start the global configuration
    Start,
    /// Stop the global configuration
    Stop,
    /// Get all services status
    Status,

    /// Run the monitor service
    Monitor {
        /// Port for the monitor
        #[arg(long)]
        monitor_port: Option<String>,
        /// Toggle the monitor service
        #[command(subcommand)]
        toggle: Option<Toggle>,
    },
    /// Run the controller service
    Controller {
        /// Host for the controller (optional)
        #[arg(long)]
        controller_host: Option<String>,
        /// Port for the controller
        #[arg(long)]
        controller_port: Option<String>,
        /// Database URL for the controller
        #[arg(long)]
        database_url: Option<String>,
        /// Toggle the controller service
        #[command(subcommand)]
        toggle: Option<Toggle>,
    },
    /// Run the release agent service
    ReleaseAgent {
        /// Host for the release agent (optional)
        #[arg(long)]
        release_agent_host: Option<String>,
        /// Port for the release agent
        #[arg(long)]
        release_agent_port: Option<String>,
        /// Passphrase for the release agent
        #[arg(long)]
        passphrase: Option<String>,
        /// Secret key for the release agent
        #[arg(long)]
        secret_key: Option<String>,
        /// Git path for the release agent
        #[arg(long)]
        git_path: Option<String>,
        /// Bucket address for the release agent
        #[arg(long)]
        bucket_addr: Option<String>,
        /// Bucket access key for the release agent
        #[arg(long)]
        bucket_access_key: Option<String>,
        /// Bucket secret key for the release agent
        #[arg(long)]
        bucket_secret_key: Option<String>,
        /// Bucket name for the release agent
        #[arg(long)]
        bucket_name: Option<String>,
        /// Toggle the release agent service
        #[command(subcommand)]
        toggle: Option<Toggle>,
    },
    /// Run the scheduler service
    Scheduler {
        /// Host for the scheduler (optional)
        #[arg(long)]
        scheduler_host: Option<String>,
        /// Port for the scheduler
        #[arg(long)]
        scheduler_port: Option<String>,
        /// Toggle the scheduler service
        #[command(subcommand)]
        toggle: Option<Toggle>,
    },
    /// Run the agent service
    Agent {
        /// Host for the agent (optional)
        #[arg(long)]
        agent_host: Option<String>,
        /// Port for the agent
        #[arg(long)]
        agent_port: Option<u32>,
        /// Toggle the agent service
        #[command(subcommand)]
        toggle: Option<Toggle>,
    },
}

impl Cli {
    fn get_config(&self) -> Result<ClientConfig, ClientError> {
        let mut sealconfig = self.sealconfig.clone();
        let default_path = PathBuf::from(SEAL_CONFIG_DEFAULT);
        if self.sealconfig == default_path {
            // If the default path is used, we assume the config file is in the user's home directory
            let home = dirs::home_dir().ok_or(ClientError::ConfigError(format!(
                "Failed find directory for seal config",
            )))?;

            sealconfig = home.join(default_path);
        }

        let mut config = File::open(&sealconfig).map_err(|_e| {
            ClientError::ConfigError(format!("Failed to open config file at {:?}", sealconfig))
        })?;

        let mut config_buf = Vec::new();
        config.read_to_end(&mut config_buf).map_err(|_e| {
            ClientError::ConfigError(format!("Failed to read config file at {:?}", sealconfig))
        })?;

        serde_yaml::from_slice(&config_buf).map_err(|e| {
            ClientError::ConfigError(format!(
                "Failed to parse config file at {:?}: {}",
                sealconfig, e
            ))
        })
    }

    async fn grpc_client(&self) -> Result<DaemonClient<Channel>, ClientError> {
        let config = self.get_config()?;
        let server = config.server;
        DaemonClient::connect(server)
            .await
            .map_err(ClientError::GrpcError)
    }
    pub async fn trigger(&self) -> Result<(), ClientError> {
        let mut client = self.grpc_client().await?;
        match &self.command {
            Commands::Status => {
                let request = Request::new(StatusRequest{ status_type: None });
                let response = client
                    .status(request)
                    .await
                    .map_err(ClientError::StatusError)?;
                println!("{}", response.into_inner());
                return Ok(());
            }
            Commands::Start => {
                println!("Starting configuration...");
                let request = Request::new(());
                let _ = client
                    .start(request)
                    .await
                    .map_err(ClientError::StatusError)?;
                println!("Global configuration started successfully.");
                return Ok(());
            }
            Commands::Stop => {
                println!("Stopping global configuration...");
                let request = Request::new(());
                let _ = client
                    .stop(request)
                    .await
                    .map_err(ClientError::StatusError)?;
                println!("Global configuration stopped successfully.");
                return Ok(());
            }
            Commands::Monitor {
                monitor_port,
                toggle,
            } => {
                if let Some(Toggle::Status) = toggle {
                    let request = Request::new(StatusRequest{ status_type: Some(Services::Monitor.into()) });
                    let response = client
                        .status(request)
                        .await
                        .map_err(ClientError::MonitorError)?;
                    println!("Monitor service status: {}", response.into_inner());
                    return Ok(());
                }
                let toggle_bool = toggle.map(|f| f.as_bool());
                let mutation = MonitorMutation {
                    monitor_port: monitor_port.to_owned(),
                    toggle_monitor: toggle_bool,
                };
                let request = Request::new(mutation);
                client
                    .mutate_monitor(request)
                    .await
                    .map_err(ClientError::MonitorError)?;
                println!(
                    "Monitor service mutation sent (port: {:?}, toggle: {:?})",
                    monitor_port, toggle_bool
                );
            }
            Commands::Controller {
                controller_host,
                controller_port,
                database_url,
                toggle,
            } => {
                if let Some(Toggle::Status) = toggle {
                    let request = Request::new(StatusRequest{ status_type: Some(Services::Controller.into()) });
                    let response = client
                        .status(request)
                        .await
                        .map_err(ClientError::ControllerError)?;
                    println!("{}", response.into_inner());
                    return Ok(());
                }
                let toggle_bool = toggle.map(|f| f.as_bool());
                let mutation = ControllerMutation {
                    controller_host: controller_host.to_owned(),
                    controller_port: controller_port.to_owned(),
                    database_url: database_url.to_owned(),
                    toggle_controller: toggle_bool,
                };
                let request = Request::new(mutation);
                client
                    .mutate_controller(request)
                    .await
                    .map_err(ClientError::ControllerError)?;
                println!(
                    "Controller service mutation sent (host: {:?}, port: {:?}, db: {:?}, toggle: {:?})",
                    controller_host, controller_port, database_url, toggle_bool
                );
            }
            Commands::ReleaseAgent {
                release_agent_host,
                release_agent_port,
                passphrase,
                secret_key,
                git_path,
                bucket_addr,
                bucket_access_key,
                bucket_secret_key,
                bucket_name,
                toggle,
            } => {
                if let Some(Toggle::Status) = toggle {
                    let request = Request::new(StatusRequest{ status_type: Some(Services::ReleaseAgent.into()) });
                    let response = client
                        .status(request)
                        .await
                        .map_err(ClientError::ReleaseAgentError)?;
                    println!("{}", response.into_inner());
                    return Ok(());
                }
                let toggle_bool = toggle.map(|f| f.as_bool());
                let mutation = ReleaseAgentMutation {
                    release_agent_host: release_agent_host.to_owned(),
                    release_agent_port: release_agent_port.to_owned(),
                    passphrase: passphrase.to_owned(),
                    secret_key: secret_key.to_owned(),
                    git_path: git_path.to_owned(),
                    bucket_addr: bucket_addr.to_owned(),
                    bucket_access_key: bucket_access_key.to_owned(),
                    bucket_secret_key: bucket_secret_key.to_owned(),
                    bucket_name: bucket_name.to_owned(),
                    toggle_release_agent: toggle_bool,
                };
                let request = Request::new(mutation);
                client
                    .mutate_release_agent(request)
                    .await
                    .map_err(ClientError::ReleaseAgentError)?;
                println!(
                    "Release agent mutation sent (host: {:?}, port: {:?}, toggle: {:?})",
                    release_agent_host, release_agent_port, toggle_bool
                );
            }
            Commands::Scheduler {
                scheduler_host,
                scheduler_port,
                toggle,
            } => {
                if let Some(Toggle::Status) = toggle {
                    let request = Request::new(StatusRequest{ status_type: Some(Services::Scheduler.into()) });
                    let response = client
                        .status(request)
                        .await
                        .map_err(ClientError::SchedulerError)?;
                    println!("{}", response.into_inner());
                    return Ok(());
                }
                let toggle_bool = toggle.map(|f| f.as_bool());
                let mutation = SchedulerMutation {
                    scheduler_host: scheduler_host.to_owned(),
                    scheduler_port: scheduler_port.to_owned(),
                    toggle_scheduler: toggle_bool,
                };
                let request = Request::new(mutation);
                client
                    .mutate_scheduler(request)
                    .await
                    .map_err(ClientError::SchedulerError)?;
                println!(
                    "Scheduler service mutation sent (host: {:?}, port: {:?}, toggle: {:?})",
                    scheduler_host, scheduler_port, toggle_bool
                );
            }
            Commands::Agent {
                agent_host,
                agent_port,
                toggle,
            } => {
                if let Some(Toggle::Status) = toggle {
                    let request = Request::new(StatusRequest{ status_type: Some(Services::Agent.into()) });
                    let response = client
                        .status(request)
                        .await
                        .map_err(ClientError::AgentError)?;
                    println!("{}", response.into_inner());
                    return Ok(());
                }
                let toggle_bool = toggle.map(|f| f.as_bool());
                let mutation = AgentMutation {
                    agent_host: agent_host.to_owned(),
                    agent_port: agent_port.to_owned(),
                    toggle_agent: toggle_bool,
                };
                let request = Request::new(mutation);
                client
                    .mutate_agent(request)
                    .await
                    .map_err(ClientError::AgentError)?;
                println!(
                    "Agent service mutation sent (host: {:?}, port: {:?}, toggle: {:?})",
                    agent_host, agent_port, toggle_bool
                );
            }
        }
        Ok(())
    }
}
