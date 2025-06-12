pub mod app_context;
pub mod http;
pub mod ports;
pub mod services;

use crate::application::app_context::AppContext;
use crate::application::http::pipeline::router::configure as configure_pipeline_routes;
use crate::application::http::release::router::configure as configure_release_routes;

use crate::config::Config;
use crate::domain::command::entities::command::CommandError;
use crate::domain::scheduler::entities::scheduler::SchedulerError;
use crate::parser::pipe_parser::ParsingError;
use crate::{docs, health};
use actix_cors::Cors;
use actix_web::dev::{Server, ServerHandle};
use actix_web::web::Data;
use actix_web::HttpServer;
use sealcid_traits::proto::ServiceStatus as Status;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use tracing::info;

#[derive(Debug)]
pub enum AppError {
    ParsingError(ParsingError),
    CommandError(CommandError),
    SchedulerError(SchedulerError),
    ActixWebError,
    Error(String),
    DatabaseConnectionError(sqlx::Error),
    SchedulerConnectionError,
    GrpcConnectionError(tonic::transport::Error),
    ReleaseConnectionError(tonic::transport::Error),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::ParsingError(_) => write!(f, "Parsing error"),
            AppError::CommandError(e) => write!(f, "Command error: {}", e),
            AppError::SchedulerError(e) => write!(f, "Scheduler error: {}", e),
            AppError::ActixWebError => write!(f, "Actix web error"),
            AppError::Error(msg) => write!(f, "Error: {}", msg),
            AppError::DatabaseConnectionError(error) => write!(f, "Database error: {}", error),
            AppError::SchedulerConnectionError => write!(f, "Scheduler connection error"),
            AppError::GrpcConnectionError(error) => write!(f, "gRPC connection error: {}", error),
            AppError::ReleaseConnectionError(error) => {
                write!(f, "Release connection error: {}", error)
            }
        }
    }
}
// type Error = AppError;

#[derive(Clone)]
pub struct App {
    config: Arc<Config>,
    app_process: Arc<RwLock<Vec<tokio::task::JoinHandle<Result<(), AppError>>>>>,
    server_handle: Arc<Mutex<Option<ServerHandle>>>,
}

impl sealcid_traits::App<Config> for App {
    type Error = AppError;

    async fn run(&self) -> Result<(), AppError> {
        let app_process = self.app_process.clone();
        let mut process = app_process.write().await;
        let mut this = self.clone();
        if !process.is_empty() {
            info!("Controller service is already running.");
            return Ok(());
        }
        process.push(tokio::spawn(async move {
            this.start()
                .await?
                .await
                .expect("should be launched successfully");
            if let Err(e) = this.start().await {
                tracing::error!("Failed to start Controller service: {}", e);
            }
            tracing::info!("Controller service stopped.");
            Ok(())
        }));
        info!("Controller service started successfully.{}", process.len());
        Ok(())
    }

    async fn configure(config: Config) -> Result<Self, AppError> {
        Self::init(config).await
    }

    async fn stop(&self) -> Result<(), AppError> {
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

    async fn configuration(&self) -> Result<impl std::fmt::Display, AppError> {
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
        "Controller".to_string()
    }
}

impl App {
    pub async fn init(config: Config) -> Result<Self, AppError> {
        // Initialize application context with database and gRPC service configurations
        Ok(Self {
            config: Arc::new(config),
            app_process: Arc::new(RwLock::new(Vec::new())),
            server_handle: Arc::new(Mutex::new(None)),
        })
    }

    pub async fn start(&self) -> Result<Server, AppError> {
        let config = Arc::clone(&self.config);
        let app_context = AppContext::initialize(
            &config.database_url,
            &config.grpc,
            &self.config.release_agent,
        )
        .await?;
        // Start HTTP server with CORS, logging middleware, and configured routes
        let server = HttpServer::new(move || {
            // Configure CORS to allow any origin/method/header, cache preflight for 1 hour
            let cors = Cors::default()
                .allow_any_origin()
                .allow_any_method()
                .allow_any_header()
                .max_age(3600);

            actix_web::App::new()
                .wrap(cors)
                .wrap(actix_web::middleware::Logger::default())
                // Register application state data for pipeline, action, and scheduler services
                .app_data(Data::new(app_context.clone()))
                .configure(configure_pipeline_routes)
                .configure(configure_release_routes)
                // Add documentation and health check endpoints
                .service(docs::doc)
                .service(docs::openapi)
                .route(
                    "/health",
                    actix_web::web::get().to(health::handlers::health_check),
                )
        })
        .bind(config.http.clone())
        .map_err(|_| AppError::ActixWebError)?
        .workers(1)
        .run();

        *self.server_handle.lock().await = Some(server.handle());

        Ok(server)
    }
}
