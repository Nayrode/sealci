use futures::lock::Mutex;
use std::sync::Arc;

use crate::
    infrastructure::{
        db::postgres::Postgres,
        grpc::grpc_scheduler_client::GrpcSchedulerClient,
        repositories::{
            action_repository::PostgresActionRepository,
            command_repository::PostgresCommandRepository, log_repository::PostgresLogRepository,
            pipeline_repository::PostgresPipelineRepository,
        },
    }
;

use super::
    services::{
        action_service::ActionServiceImpl, command_service::CommandServiceImpl,
        pipeline_service::PipelineServiceImpl, scheduler_service_impl::SchedulerServiceImpl,
    }
;

#[derive(Clone)]
pub struct AppContext {
    pub pipeline_service: Arc<PipelineServiceImpl<PostgresPipelineRepository, PostgresLogRepository, ActionServiceImpl<PostgresActionRepository, CommandServiceImpl<PostgresCommandRepository>>, SchedulerServiceImpl<ActionServiceImpl<PostgresActionRepository, CommandServiceImpl<PostgresCommandRepository>>, GrpcSchedulerClient, PostgresPipelineRepository>>>,
    pub action_service: Arc<ActionServiceImpl<PostgresActionRepository, CommandServiceImpl<PostgresCommandRepository>>>,
    pub scheduler_service: Arc<Mutex<SchedulerServiceImpl<ActionServiceImpl<PostgresActionRepository, CommandServiceImpl<PostgresCommandRepository>>, GrpcSchedulerClient, PostgresPipelineRepository>>>,
}

impl AppContext {
    pub async fn initialize(database_url: &str, grpc_url: &str) -> Self {
        // Initialize Postgres connection pool using provided database URL
        let postgres = Arc::new(Postgres::new(database_url).await);

        // Create gRPC client for scheduler service
        let grpc_client = GrpcSchedulerClient::new(grpc_url)
            .await
            .expect("Failed to connect to scheduler");

        // Wrap gRPC client in async Mutex for shared state
        let scheduler_client = Arc::new(Mutex::new(grpc_client));

        let command_repository = Arc::new(PostgresCommandRepository::new(postgres.clone()));

        let action_repository = Arc::new(PostgresActionRepository::new(postgres.clone()));

        let command_service = Arc::new(CommandServiceImpl::new(command_repository));

        let action_service = Arc::new(ActionServiceImpl::new(action_repository, command_service));
        let pipeline_repository = Arc::new(PostgresPipelineRepository::new(postgres.clone()));

        let log_repository = Arc::new(PostgresLogRepository::new(postgres.clone()));

        let scheduler_service = Arc::new(Mutex::new(SchedulerServiceImpl::new(
            action_service.clone(),
            scheduler_client,
            pipeline_repository.clone(),
        )));

        let pipeline_service = Arc::new(PipelineServiceImpl::new(
            pipeline_repository.clone(),
            log_repository.clone(),
            action_service.clone(),
            scheduler_service.clone(),
        ));

        Self {
            pipeline_service,
            action_service,
            scheduler_service,
        }
    }
}
