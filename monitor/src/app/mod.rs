use std::sync::Arc;

use actix_web::{web::Data, App as ActixApp, HttpServer};

use actix_web::dev::{Server, ServerHandle};
use tokio::sync::{Mutex, RwLock};

use actix_cors::Cors;

use tracing::info;
use sealcid_traits::proto::ServiceStatus as Status;
use crate::{
    config::Config,
    error::Error,
    external_api::{
        add_configuration, delete_configuration, doc, get_actions_file, get_configuration_by_id,
        get_configurations, update_configuration,
    },
    github::GitHubClient,
    service::listener::ListenerService,
};

#[derive(Clone)]
pub struct App {
    config: Arc<Config>,
    app_process: Arc<RwLock<Vec<tokio::task::JoinHandle<Result<(), Error>>>>>,
    server_handle: Arc<Mutex<Option<ServerHandle>>>,
}

impl sealcid_traits::App<Config> for App {
    type Error = Error;

    async fn run(&self) -> Result<(), Error> {
            let app_process = self.app_process.clone();
            let mut process = app_process.write().await;
            let mut this = self.clone();
            if !process.is_empty() {
                info!("Monitor service is already running.");
                return Ok(());
            }
            process.push(tokio::spawn(async move {
                this.start().await?.await.expect("should be launched successfully");
                if let Err(e) = this.start().await {
                    tracing::error!("Failed to start Monitor service: {}", e);
                }
                tracing::info!("Monitor service stopped.");
                Ok(())
            }));
            info!("Monitor service started successfully.{}", process.len());
            Ok(())
        }

    async fn configure(config: Config) -> Result<Self, Error> {
        Self::init(config).await
    }

    async fn stop(&self) -> Result<(), Error> {
        let app_process = self.app_process.clone();
        let handle = app_process.write().await.pop();
        if let Some(handle) = handle {
            if handle.is_finished() {
                info!("Service is already finished.");
            } else {
                if let Some(server_handle) = self.server_handle.lock().await.as_ref() {
                    // Attempt to stop the server gracefully
                    server_handle.stop(false).await;
                } else {
                    info!("No server handle available to stop.");
                }
                handle.abort();
                info!("Service abort requested.");
            }
        } else {
            info!("No service to stop.");
        }
        Ok(())
    }

    async fn configuration(&self) -> Result<impl std::fmt::Display, Error> {
        Ok(self.config.clone())
    }

    async fn status(&self) -> Status {
            let guard = self.app_process.read().await;
            let app_process = guard.get(0);
            match app_process {
                Some(handle) => {
                    if handle.is_finished() {
                        Status::Stopped
                    } else {
                        Status::Running
                    }
                }
                None => Status::Stopped,
            }
        }

    fn name(&self) -> String {
        "Monitor".to_string()
    }
}

impl App {
    pub async fn init(config: Config) -> Result<Self, Error> {
            Ok(App {
                config: Arc::new(config),
                app_process: Arc::new(RwLock::new(Vec::new())),
                server_handle: Arc::new(Mutex::new(None)),
            })
        }

    pub async fn start(&self) -> Result<Server, Error> {
        // Initialize the application, set up routes, etc.
        info!("Application is running...");
        let github_client = Arc::new(GitHubClient::new());
        let controller_client = Arc::new(crate::controller::ControllerClient::new(
            self.config.controller_host.clone(),
        ));
        let listener_service = Arc::new(ListenerService::new(
            github_client.clone(),
            controller_client.clone(),
        ));
        let server = HttpServer::new({
            move || {
                let cors = Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600);

                ActixApp::new()
                    .wrap(cors)
                    .wrap(actix_web::middleware::Logger::default())
                    .app_data(Data::new(listener_service.clone()))
                    .service(get_configurations)
                    .service(get_configuration_by_id)
                    .service(add_configuration)
                    .service(update_configuration)
                    .service(delete_configuration)
                    .service(get_actions_file)
                    .service(doc)
            }
        })
        .bind(("0.0.0.0", self.config.clone().port))
        .map_err(Error::ServerError)?
        .run();

        *self.server_handle.lock().await = Some(server.handle());
        Ok(server)
    }
}
