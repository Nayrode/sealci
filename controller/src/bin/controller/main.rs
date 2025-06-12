use actix_cors::Cors;
use actix_web::{web::Data, App, HttpServer};
use clap::Parser;
use controller::application::app_context::AppContext;
use controller::application::http::pipeline::router::configure as configure_pipeline_routes;
use controller::config::Config;
use controller::application::http::release::router::configure as configure_release_routes;
use controller::{docs, health};
use dotenv::dotenv;
use tracing::info;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load .env file into OS environment variables before parsing clap args.
    // This ensures HTTP, DATABASE_URL, GRPC can be defined in .env and picked up here.
    dotenv().ok();

    // Parse command line arguments and environment variables into Args struct
    let args = Config::parse();

    // Debug print the provided arguments for verification
    println!("Parsed args: {:?}", args);

    // HTTP server binding address (host:port)
    let addr_in: String = args.http;

    // Initialize application context with database and gRPC service configurations
    let app_context: AppContext =
        AppContext::initialize(&args.database_url, &args.grpc, &args.release_agent)
            .await
            .expect("REASON");

    // Initialize tracing subscriber for logging
    tracing_subscriber::fmt::init();

    info!("Listening on {}", addr_in);

    // Start HTTP server with CORS, logging middleware, and configured routes
    HttpServer::new(move || {
        // Configure CORS to allow any origin/method/header, cache preflight for 1 hour
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
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
    .bind(addr_in)?
    .workers(1)
    .run()
    .await
}
