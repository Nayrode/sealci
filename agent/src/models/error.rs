use tokio::task::JoinError;
use tonic::Status;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Error {
    // Define your custom error variants here\
    Error(String),
    DockerConnectionError(bollard::errors::Error),
    ContainerStartError(bollard::errors::Error),
    PullImageError(bollard::errors::Error),
    ContainerRemoveError(bollard::errors::Error),
    ContainerExecError(bollard::errors::Error),
    ContainerExecDetachedError,
    ExecError(JoinError),
    StepOutputError(i32),
    ConnectionError(tonic::transport::Error),
    ServeError(tonic::transport::Error),
    RegistrationError(Status),
    ReportHealthError(Status),
    NotRegisteredError,
    HealthStreamError,
    ActionNotFound,
    ActionStateError,
    BrokerSendError(String),
    ChannelError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Error(msg) => write!(f, "Error: {}", msg),
            Error::DockerConnectionError(e) => write!(f, "Docker connection error: {}", e),
            Error::ContainerStartError(e) => write!(f, "Container start error: {}", e),
            Error::PullImageError(e) => write!(f, "Pull image error: {}", e),
            Error::ContainerRemoveError(e) => write!(f, "Container remove error: {}", e),
            Error::ContainerExecError(e) => write!(f, "Container exec error: {}", e),
            Error::ContainerExecDetachedError => write!(f, "Container exec detached error"),
            Error::ExecError(e) => write!(f, "Exec error: {}", e),
            Error::StepOutputError(code) => write!(f, "Step output error with code: {}", code),
            Error::ConnectionError(e) => write!(f, "Connection error: {}", e),
            Error::ServeError(e) => write!(f, "Serve error: {}", e),
            Error::RegistrationError(status) => write!(f, "Registration error: {}", status),
            Error::ReportHealthError(status) => write!(f, "Report health error: {}", status),
            Error::NotRegisteredError => write!(f, "Not registered error"),
            Error::HealthStreamError => write!(f, "Health stream error"),
            Error::ActionNotFound => write!(f, "Action not found"),
            Error::ActionStateError => write!(f, "Action state error"),
            Error::BrokerSendError(msg) => write!(f, "Broker send error: {}", msg),
            Error::ChannelError(msg) => write!(f, "Channel error: {}", msg),
        }
    }
}
