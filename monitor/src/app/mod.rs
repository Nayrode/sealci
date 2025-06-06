use std::sync::Arc;

use actix_web::{web::Data, App as ActixApp, HttpServer};

use actix_web::dev::Server;
use tokio::sync::RwLock;

use actix_cors::Cors;

use tracing::info;
use sealcid_traits::status::Status;
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
    listener_service: Arc<ListenerService>,
    app_process: Arc<RwLock<tokio::task::JoinHandle<Result<(), Error>>>>,
}

impl sealcid_traits::App<Config> for App {
    type Error = Error;

    async fn run(&self) -> Result<(), Error> {
            let app_process = self.app_process.clone();
            let app_clone = self.clone();
            let mut process = app_process.write().await;
            *process = tokio::spawn(async move {
                info!("Starting Monitor service...");

                let server_result = app_clone.start().await;
                match server_result {
                    Ok(server) => {
                        match server.await {
                            Ok(_) => {
                                info!("App started successfully");
                            }
                            Err(e) => {
                                tracing::error!("Failed to run server: {:?}", e);
                                return Err(Error::ServerError(e));
                            }
                        }
                    }
                    Err(e) => {
                        tracing::error!("Failed to start app: {:?}", e);
                        return Err(e);
                    }
                }
                Ok(())
            });
            Ok(())
        }

    async fn configure(config: Config) -> Result<Self, Error> {
        Self::init(config).await
    }

    async fn stop(&self) -> Result<(), Error> {
        let app_process = self.app_process.clone();
        let process = app_process.read().await;
        process.abort();
        Ok(())
    }

    async fn configuration(&self) -> Result<impl std::fmt::Display, Error> {
        Ok(self.config.clone())
    }

    async fn status(&self) -> Status {
        let app_process = self.app_process.read().await;
        if app_process.is_finished() {
            // Try to get the result without blocking
            Status::Stopped
        } else {
            Status::Running
        }
    }

    fn name(&self) -> String {
        "Monitor".to_string()
    }
}

impl App {
    pub async fn init(config: Config) -> Result<Self, Error> {
        let github_client = Arc::new(GitHubClient::new());
        let controller_client = Arc::new(crate::controller::ControllerClient::new(
            config.controller_host.clone(),
        ));
        let listener_service = Arc::new(ListenerService::new(
            github_client.clone(),
            controller_client.clone(),
        ));
        Ok(App {
            config: Arc::new(config),
            listener_service,
            app_process: Arc::new(RwLock::new(tokio::task::spawn(async {
                Ok(())
            }))),
        })
    }

    pub async fn start(&self) -> Result<Server, Error> {
        // Initialize the application, set up routes, etc.
        info!("Application is running...");
        let listener_service = Arc::clone(&self.listener_service.clone());
        Ok(HttpServer::new({

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
        .run())
    }
}