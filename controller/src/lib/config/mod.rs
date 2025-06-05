use std::fmt::Display;
use clap::Parser;

#[derive(Parser, Debug, Clone)]
pub struct Config {
    /// HTTP listen address (host:port).
    /// Precedence (see clap docs: defaults < env < CLI):
    /// 1. --http CLI flag
    /// 2. HTTP env var (from OS or .env)
    /// If unset, clap will report an error.
    /// Example values:
    ///   --http 127.0.0.1:8080
    ///   HTTP=0.0.0.0:8000
    #[clap(env, long)]
    pub http: String,

    /// Database connection string.
    /// Precedence:
    /// 1. --database-url CLI flag
    /// 2. DATABASE_URL env var
    /// If unset, clap will report an error.
    /// Example:
    ///   --database-url postgres://user:pass@localhost:5432/mydb
    #[clap(env, long)]
    pub database_url: String,

    /// gRPC scheduler service endpoint.
    /// Precedence:
    /// 1. --grpc CLI flag
    /// 2. GRPC env var
    /// If unset, clap will report an error.
    /// Example:
    ///   --grpc http://127.0.0.1:50051
    #[clap(env, long)]
    pub grpc: String,
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "--http {}, --database_url {}, --grpc {}",
            self.http, self.database_url, self.grpc
        )
    }
}