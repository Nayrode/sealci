mod config;
mod constants;
mod controller;
mod event_listener;
mod external_api;
mod file_utils;
mod github;

use crate::constants::SERVER_ADDRESS;
use crate::external_api::{
    add_configuration, delete_configuration, get_actions_file, get_configuration_by_id,
    get_configurations, update_configuration,
};
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use std::sync::Arc;
use tokio;
use tokio::sync::RwLock;
use tokio::task::JoinSet;
use tracing::info;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let configs = Arc::new(RwLock::new(config));

    let thread_listeners_handles = Arc::new(RwLock::new(JoinSet::new()));

    info!("Launching API and listening to events on GitHub repository...");

    // Launch the API server as the main task
    launch_api_server(Arc::clone(&configs), Arc::clone(&thread_listeners_handles)).await?;

    Ok(())
}

async fn launch_api_server(
    configs: Arc<RwLock<Config>>,
    thread_listeners_handles: Arc<RwLock<JoinSet<()>>>,
) -> std::io::Result<()> {
    let data = web::Data::new(AppState {
        configs: Arc::clone(&configs),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .app_data(Data::from(Arc::clone(&thread_listeners_handles)))
            .service(get_configurations)
            .service(get_configuration_by_id)
            .service(add_configuration)
            .service(update_configuration)
            .service(delete_configuration)
            .service(get_actions_file)
    })
    .bind(SERVER_ADDRESS)?
    .run()
    .await
}
