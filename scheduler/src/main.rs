use clap::Parser;

#[derive(Debug, Clone, Parser)]
struct Config {
    #[clap(short, long, default_value = "0.0.0.0:50051", 
            help = "The address to bind the gRPC server to")]
    pub addr: String,
}

#[tokio::main]
async fn main() -> Result<(), sealci_scheduler::errors::Error> {
    tracing_subscriber::fmt::init();

    let config = Config::parse();

    let app = sealci_scheduler::app::App::new(sealci_scheduler::app::Config{
        addr: config.addr,
    });

    app.run().await?;

    Ok(())
}
