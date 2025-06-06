use clap::Parser;

#[derive(Parser, Debug)]
struct Config {
    #[arg(short, long, default_value_t = String::from("[::0]:50051"))]
    addr: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initializes the Agent Pool and Action queue. They are lost when the Scheduler dies.
    let config = Config::parse();
    let app = sealci_scheduler::app::App::init(sealci_scheduler::app::Config{
        addr: config.addr,
    });
    
    Ok(())
}
