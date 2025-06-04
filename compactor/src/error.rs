use dumplet::DumpletError;

#[derive(Debug)]
pub enum Error {
    DumpletError(DumpletError),
    DumperError(dumper::common::error::Error),
}
