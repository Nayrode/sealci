use std::fmt;

#[derive(Debug)] // Added Clone here
pub enum Error {
    GrpcSetupError(tonic::Status),
    GrpcServerError(tonic::Status),
    GrpcClientError(tonic::Status),
    GrpcRequestError(tonic::Status),
    AddrParseError(std::net::AddrParseError),
    InvalidAgentHostError(String),

    OtherError(String),
}

impl From<Error> for tonic::Status {
    fn from(err: Error) -> Self {
        match err {
            Error::GrpcRequestError(status) => status,
            Error::GrpcSetupError(status) => status,
            Error::GrpcServerError(status) => status,
            Error::GrpcClientError(status) => status,

            Error::OtherError(msg) => tonic::Status::internal(msg),
            _ => tonic::Status::unknown("An unknown error occurred"),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::GrpcSetupError(status) => write!(f, "GrpcSetupError: {}", status.message()),
            Error::GrpcServerError(status) => write!(f, "GrpcServerError: {}", status.message()),
            Error::GrpcClientError(status) => write!(f, "GrpcClientError: {}", status.message()),
            Error::GrpcRequestError(status) => write!(f, "GrpcRequestError: {}", status.message()),
            Error::AddrParseError(err) => write!(f, "AddrParseError: {}", err),
            Error::InvalidAgentHostError(msg) => write!(f, "InvalidAgentHostError: {}", msg),

            Error::OtherError(msg) => write!(f, "OtherError: {}", msg),
        }
    }
}
