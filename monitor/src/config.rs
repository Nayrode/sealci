use clap::Parser;

#[derive(Parser, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// The host URL of the controller
    #[clap(long, default_value = "http://0.0.0.0:5001", env = "CONTROLLER_HOST")]
    pub controller_host: String,

    /// The port of the agent to listen on
    #[clap(long, default_value = "9001", env = "PORT")]
    pub port: u16,
}
