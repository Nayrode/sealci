use std::fmt::Display;

pub enum Error {
    Unhandled,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Error::Unhandled => write!(f, "Unhandled error"),
            }
        }
}
