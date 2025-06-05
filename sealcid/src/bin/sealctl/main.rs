use clap::Parser;
use miette::{Report, Result as MietteResult};
use sealcid::client::commands::Cli;

#[tokio::main]
async fn main() -> MietteResult<()> {
    let config = Cli::parse();
    match config.trigger().await {
        Ok(_) => println!("Command executed successfully."),
        Err(e) => {
            return Err(Report::new(e));
        }
    };
    Ok(())
}
