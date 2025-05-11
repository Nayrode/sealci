use actix_cors::Cors;
use actix_web::{web::Data, App, HttpServer};
use clap::Parser;
use controller::application::http::pipeline::router::configure as configure_pipeline_routes;
use controller::application::services::action_service::ActionServiceImpl;
use controller::application::services::command_service::CommandServiceImpl;
use controller::application::services::pipeline_service::PipelineServiceImpl;
use controller::application::services::scheduler_service_impl::SchedulerServiceImpl;
use controller::infrastructure::db::postgres::Postgres;
use controller::infrastructure::grpc::grpc_scheduler_client::GrpcSchedulerClient;
use controller::infrastructure::repositories::action_repository::PostgresActionRepository;
use controller::infrastructure::repositories::command_repository::PostgresCommandRepository;
use controller::infrastructure::repositories::pipeline_repository::PostgresPipelineRepository;
use controller::{docs, health};
use dotenv::dotenv;
use futures::lock::Mutex;
use std::sync::Arc;
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

    tracing_subscriber::fmt::init();

    let postgres = Arc::new(Postgres::new(&args.database_url).await);

    let addr_in: String = args.http;

    let grpc_client= 
        GrpcSchedulerClient::new(&args.grpc)
            .await
            .expect("Failed to connect to scheduler");
    let scheduler_client = Arc::new(Mutex::new(grpc_client));

    let command_repository = Arc::new(PostgresCommandRepository::new(postgres.clone()));

    let action_repository = Arc::new(PostgresActionRepository::new(postgres.clone()));

    let command_service =
        Arc::new(CommandServiceImpl::new(command_repository));

    let action_service = Arc::new(
        ActionServiceImpl::new(action_repository, command_service),
    );

    let pipeline_repository = Arc::new(PostgresPipelineRepository::new(postgres.clone()));
    
    let scheduler_service =
    Arc::new(Mutex::new(SchedulerServiceImpl::new(
        action_service.clone(),
        scheduler_client,
        pipeline_repository.clone(),
    )));

    let pipeline_service = Arc::new(
        PipelineServiceImpl::new(pipeline_repository.clone(), action_service.clone(), scheduler_service.clone()),
    );

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
            .app_data(Data::new(pipeline_service.clone()))
            .app_data(Data::new(action_service.clone()))
            .app_data(Data::new(scheduler_service.clone()))
            .configure(configure_pipeline_routes)
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