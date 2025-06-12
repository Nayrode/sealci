use tonic::Status;
use thiserror::Error;
use miette::Diagnostic;

#[derive(Debug, Error, Diagnostic)]
pub enum ClientError {
    #[error("Monitor error: {0}")]
    #[diagnostic(code(client_error::monitor))]
    MonitorError(Status),

    #[error("Controller error: {0}")]
    #[diagnostic(code(client_error::controller))]
    ControllerError(Status),

    #[error("Release agent error: {0}")]
    #[diagnostic(code(client_error::release_agent))]
    ReleaseAgentError(Status),

    #[error("Scheduler error: {0}")]
    #[diagnostic(code(client_error::scheduler))]
    SchedulerError(Status),

    #[error("Agent error: {0}")]
    #[diagnostic(code(client_error::agent))]
    AgentError(Status),

    #[error("Configuration error: {0}")]
    #[diagnostic(code(client_error::config))]
    ConfigError(String),

    #[error("gRPC transport error: {0}")]
    #[diagnostic(code(client_error::grpc))]
    GrpcError(tonic::transport::Error),

    #[error("Status error: {0}")]
    #[diagnostic(code(client_error::grpc))]
    StatusError(Status),
}
