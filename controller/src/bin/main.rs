use actix_cors::Cors;
use actix_web::{web::Data, App, HttpServer};
use clap::Parser;
use controller::application::ports::action_service::ActionService;
use controller::application::ports::command_service::CommandService;
use controller::application::ports::pipeline_service::PipelineService;
use controller::application::ports::scheduler_service::SchedulerService;
use controller::application::services::action_service::ActionServiceImpl;
use controller::application::services::command_service::CommandServiceImpl;
use controller::application::services::pipeline_service::PipelineServiceImpl;
use controller::application::services::scheduler_service_impl::SchedulerServiceImpl;
use controller::domain::action::ports::action_repository::ActionRepository;
use controller::domain::command::ports::command_repository::CommandRepository;
use controller::domain::pipeline::ports::pipeline_repository::PipelineRepository;
use controller::domain::scheduler::services::scheduler_client::SchedulerClient;
use controller::infrastructure::db::postgres::Postgres;
use controller::infrastructure::grpc::grpc_scheduler_client::GrpcSchedulerClient;
use controller::infrastructure::repositories::action_repository::PostgresActionRepository;
use controller::infrastructure::repositories::command_repository::PostgresCommandRepository;
use controller::infrastructure::repositories::pipeline_repository::PostgresPipelineRepository;
use controller::parser::pipe_parser::PipeParser;
use controller::application::http::pipeline::router::configure as configure_pipeline_routes;
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

    let postgres = Arc::new(Postgres::new(&args.database_url).await);

    let addr_in: String = args.http;
    let grpc_scheduler = args.grpc;

    tracing_subscriber::fmt::init();

    let command_repository = Arc::new(Box::new(PostgresCommandRepository::new(postgres.clone()))
        as Box<dyn CommandRepository + Send + Sync>);

    let action_repository = Arc::new(Box::new(PostgresActionRepository::new(postgres.clone()))
        as Box<dyn ActionRepository + Send + Sync>);

    let command_service: Arc<Box<dyn CommandService + Send + Sync>> =
        Arc::new(Box::new(CommandServiceImpl::new(command_repository))
            as Box<dyn CommandService + Send + Sync>);

    let action_service: Arc<Box<dyn ActionService + Send + Sync>> = Arc::new(Box::new(
        ActionServiceImpl::new(action_repository, Arc::clone(&command_service)),
    )
        as Box<dyn ActionService + Send + Sync>);

    let grpc_client = Box::new(
        GrpcSchedulerClient::new(&grpc_scheduler.clone())
            .await
            .expect("Failed to connect to scheduler"),
    );
    let scheduler_client = Arc::new(Mutex::new(
        grpc_client as Box<dyn SchedulerClient + Send + Sync>,
    ));
    let pipeline_repository = Arc::new(PostgresPipelineRepository::new(postgres.clone()))
        as Arc<dyn PipelineRepository + Send + Sync>;

    let scheduler_service: Arc<Box<dyn SchedulerService + Send + Sync>> =
        Arc::new(Box::new(SchedulerServiceImpl::new(
            action_service.clone(),
            scheduler_client,
            Arc::new(PostgresPipelineRepository::new(postgres.clone()))
                as Arc<dyn PipelineRepository + Send + Sync>,
        )));

    let parser_service = Arc::new(PipeParser {});

    let pipeline_repository =
    Arc::new(
        Box::new(PostgresPipelineRepository::new(postgres.clone()))
            as Box<dyn PipelineRepository + Send + Sync>
    );
    let pipeline_service = Arc::new(PipelineServiceImpl::new(
        pipeline_repository,
        Arc::clone(&action_service),
    )) as Arc<dyn PipelineService + Send + Sync>;

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
            .app_data(Data::new(Arc::clone(&action_service)))
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
