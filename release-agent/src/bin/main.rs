use clap::Parser;
use std::{path::PathBuf, sync::Arc};
use sealci_release_agent::{
    app::AppConfig, bucket::minio::MinioClient, compress::Flate2Client, core::{ReleaseAgentError}, git::Git2Client, sign::SequoiaPGPManager
};

fn rand_string(len: usize) -> String {
    use rand::{distr::Alphanumeric, rng, Rng};
    rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

#[derive(Debug, Parser)]
#[clap(
    name = "release-agent", 
    version,
    about = "Release agent for sealci",
    long_about = "The release agent is responsible for releasing artifacts and signing them in a secure way."
)]
struct Config {
    #[clap(
        env,
        long, 
        default_value_t = String::from("[::1]:50052"),
        help = "gRPC listen address",
        long_help = "Endpoint for the gRPC API of the release agent."
    )]
    pub grpc: String,

    #[clap(
        env,
        short,
        long,
        default_value_t = rand_string(32),
        help = "Passphrase for the private key",
        long_help = "Passphrase that is used to encrypt the private key, by default a random string is generated."
    )]
    pub passphrase: String,

    #[clap(
        env,
        short,
        long,
        help = "Path to store certificates",
        long_help = "Path to store certificates for each release, the destination folder is recommended to be a temporary filesystem like /tmp"
    )]
    pub cert_path: PathBuf,

    #[clap(
        env,
        long,
        help = "Path to store git repositories, signatures and compressed files",
        long_help = "Path to store git repositories for each release, the destination folder is recommended to be a temporary filesystem like /tmp but it should also be a different destination from the cert_path."
    )]
    pub git_path: PathBuf,

    #[clap(
        env,
        long,
        default_value_t = String::from("http://127.0.0.1:9000"),
        help = "Address of the bucket",
        long_help = "Address of the bucket to store releases, the default value is http://127.0.0.1:9000"
    )]
    pub bucket_addr: String,

    #[clap(
        env,
        long,
        help = "Access key for the bucket",
        long_help = "Access key for the bucket to store releases"
    )]
    pub bucket_access_key: String,

    #[clap(
        env,
        long,
        help = "Secret key for the bucket",
        long_help = "Secret key for the bucket to store releases"
    )]
    pub bucket_secret_key: String,

    #[clap(
        env,
        long,
        help = "Name of the bucket",
        long_help = "Name of the bucket to store releases"
    )]
    pub bucket_name: String,

}

#[tokio::main]
async fn main() -> Result<(), ReleaseAgentError> {
    let config = Config::parse();
    tracing_subscriber::fmt().init();

    // add signer
    let signer = SequoiaPGPManager::new(config.cert_path, config.passphrase)?;
    // add bucket
    let bucket_client = MinioClient::new(config.bucket_addr, config.bucket_access_key, config.bucket_secret_key, config.bucket_name).await?;
    // add git
    let git_client = Git2Client::new(config.git_path);
    // add compress
    let compress_client = Flate2Client::new();

    let core = sealci_release_agent::core::ReleaseAgent::new(
        signer.clone(),
        bucket_client.clone(),
        git_client.clone(),
        compress_client.clone(),
    );
    let release_agent_grpc = sealci_release_agent::grpc::ReleaseAgentService::new(Arc::new(core), signer, bucket_client, git_client, compress_client);
    let app =
        sealci_release_agent::app::App::new(
            AppConfig { grpc: config.grpc },
            release_agent_grpc,
        );

    app.run().await?;

    Ok(())
}
