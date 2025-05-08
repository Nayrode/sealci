use controller::action::action_service::ActionService;
use clap::Parser;
use controller::command::command_service::CommandService;
use controller::server::grpc_scheduler;
use controller::{docs, health, logs};
use std::sync::Arc;
use tokio::sync::Mutex;

use controller::database::database::Database;
use actix_cors::Cors;
use actix_web::{web::Data, App, HttpServer};
use dotenv::dotenv;
use controller::parser::pipe_parser::PipeParser;
use controller::pipeline::pipeline_controller;
use tracing::info;

#[derive(Parser, Debug)]
struct Args {
    #[clap(env, long)]
    pub http: String,

    #[clap(env, long)]
    pub database_url: String,

    #[clap(env, long)]
    pub grpc: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let args = Args::parse();

    println!("${:?}", args);
    let database = Database::new(&args.database_url).await;

    let pool = database.pool;

    let addr_in: String = args.http;
    let grpc_scheduler = args.grpc;

    tracing_subscriber::fmt::init();

    let client = Arc::new(Mutex::new(
        grpc_scheduler::controller_client::ControllerClient::connect(grpc_scheduler)
            .await
            .expect("Failed to connect to controller"),
    ));

    let command_service = Arc::new(CommandService::new(pool.clone()));

    let action_service = Arc::new(ActionService::new(
        pool.clone(),
        Arc::clone(&command_service),
    ));

    let scheduler_service = Arc::new(controller::scheduler::SchedulerService::new(
        client.clone(),
        Arc::new(logs::log_repository::LogRepository::new(pool.clone())),
        Arc::clone(&action_service),
    ));

    let parser_service = Arc::new(PipeParser {});

    let pipeline_service = Arc::new(controller::pipeline::pipeline_service::PipelineService::new(
        scheduler_service.clone(),
        parser_service.clone(),
        pool,
        Arc::clone(&action_service),
    ));

    info!("Listening on {}", addr_in);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(actix_web::middleware::Logger::default())
            .app_data(Data::new(pipeline_service.clone())) // TODO: replace this implementation by the real parser
            .app_data(Data::new(Arc::clone(&action_service)))
            .service(pipeline_controller::create_pipeline)
            .service(pipeline_controller::get_pipelines)
            .service(pipeline_controller::get_pipeline)
            .service(docs::doc)
            .service(docs::openapi)
            .route(
                "/health",
                actix_web::web::get().to(health::handlers::health_check),
            )
    })
    .bind(addr_in)?
    .workers(1)
    .run()
    .await
}
