use clap::Parser;

#[derive(Parser, Debug)]
pub struct Config {
    #[clap(long)]
    pub controller_url: String,

    #[clap(long)]
    pub port: u16,
}
