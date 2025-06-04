use std::fmt::Display;

use crate::common::{error::Error, status::Status};

pub trait App {
    /// Runs the application.
    /// It should store a thread handler like `JoinHandle` or similar
    /// It should not block the current thread.
    fn run(&mut self) -> impl Future<Output = Result<(), Error>>;

    /// Configures the application with the provided configuration.
    fn configure<T>(&mut self, config: T) -> impl Future<Output = Result<(), Error>>;

    /// Stops the application by aborting the running thread or process.
    fn stop(&mut self) -> impl Future<Output = Result<(), Error>>;

    /// Those methods are used to get the state of the application.
    /// This should return a displayable configuration. (Not the secret !!!)
    fn configuration(&self) -> Result<impl Display, Error>;

    /// Returns the current status of the application.
    fn status(&self) -> impl Future<Output = Result<Status, Error>>;

    fn name(&self) -> String;
}
