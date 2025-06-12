use dumplet::DumpletError;

#[derive(Debug)]
pub enum Error {
    DumpletError(DumpletError),
    DumperError(dumper::common::error::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::DumpletError(e) => write!(f, "Dumplet error: {}", e),
            Error::DumperError(e) => write!(f, "Dumper error: {}", e),
        }
    }
}