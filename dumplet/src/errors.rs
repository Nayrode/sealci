use std::{convert::Infallible, fmt};

#[derive(Debug)]
pub enum DumpletError {
    DockerError(bollard::errors::Error),
    IoError(std::io::Error),
    ParseError(Infallible),
    InvalidFormat,

    InvalidPath,
}

impl fmt::Display for DumpletError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DumpletError::DockerError(err) => write!(f, "Docker error: {}", err),
            DumpletError::IoError(err) => write!(f, "I/O error: {}", err),
            DumpletError::ParseError(infallible) => write!(f, "Parse error: {:?}", infallible),
            DumpletError::InvalidFormat => write!(f, "Invalid format encountered"),
            DumpletError::InvalidPath => write!(f, "Invalid path provided"),
        }
    }
}

impl std::error::Error for DumpletError {}

impl From<bollard::errors::Error> for DumpletError {
    fn from(err: bollard::errors::Error) -> Self {
        DumpletError::DockerError(err)
    }
}

impl From<std::io::Error> for DumpletError {
    fn from(err: std::io::Error) -> Self {
        DumpletError::IoError(err)
    }
}
