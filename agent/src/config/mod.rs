use std::fmt::Display;

use clap::Parser;

#[derive(Parser, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// The host URL of the scheduler
    #[clap(long, default_value = "http://0.0.0.0:5001")]
    pub shost: String,

    /// The host URL you want the scheduler to contact the agent on
    #[clap(long, default_value = "http://[::1]")]
    pub ahost: String,

    /// The port of the agent to listen on
    #[clap(long, default_value = "9001")]
    pub port: u32,
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "--shost {}, --ahost {}, --port {}",
            self.shost, self.ahost, self.port
        )
    }
}
