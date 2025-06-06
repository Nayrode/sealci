pub mod app_context;
pub mod http;
pub mod ports;
pub mod services;

use crate::application::app_context::AppContext;
use crate::application::http::pipeline::router::configure as configure_pipeline_routes;
use crate::config::Config;
use crate::domain::command::entities::command::CommandError;
use crate::domain::scheduler::entities::scheduler::SchedulerError;
use crate::parser::pipe_parser::ParsingError;
use crate::{docs, health};
use actix_cors::Cors;
use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::HttpServer;
use sealcid_traits::status::Status;
use std::sync::Arc;
use tokio::sync::RwLock;

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
        }
    }
}
// type Error = AppError;

#[derive(Clone)]
pub struct App {
    config: Arc<Config>,
    app_process: Arc<RwLock<tokio::task::JoinHandle<Result<(), AppError>>>>,
}

impl sealcid_traits::App<Config> for App {
    type Error = AppError;

    async fn run(&self) -> Result<(), AppError> {
        let app_process = self.app_process.clone();
        let app_clone = self.clone();
        let mut process = app_process.write().await;
        *process = tokio::spawn(async move {
            let _ = app_clone.start().await?.await;
            Ok(())
        });
        Ok(())
    }

    async fn configure(config: Config) -> Result<Self, AppError> {
        Self::init(config).await
    }

    async fn stop(&self) -> Result<(), AppError> {
        let app_process = self.app_process.clone();
        let process = app_process.read().await;
        process.abort();
        Ok(())
    }

    async fn configuration(&self) -> Result<impl std::fmt::Display, AppError> {
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
        "Controller".to_string()
    }
}

impl App {
    pub async fn init(config: Config) -> Result<Self, AppError> {
        // Initialize tracing subscriber for logging
        tracing_subscriber::fmt::init();

        // Initialize application context with database and gRPC service configurations
        Ok(Self {
            config: Arc::new(config),
            app_process: Arc::new(RwLock::new(tokio::spawn(async { Ok(()) }))),
        })
    }

    pub async fn start(&self) -> Result<Server, AppError> {
        let app_context: AppContext =
            AppContext::initialize(&self.config.database_url, &self.config.grpc).await?;
        let config = Arc::clone(&self.config);
        // Start HTTP server with CORS, logging middleware, and configured routes
        Ok(HttpServer::new(move || {
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
        .run())
    }
}
