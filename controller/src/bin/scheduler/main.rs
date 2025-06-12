use controller::server::{
    grpc_scheduler::controller_server::ControllerServer, MockSchedulerService,
};
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse().unwrap();
    let scheduler_service = MockSchedulerService {};

    Server::builder()
        .add_service(ControllerServer::new(scheduler_service))
        .serve(addr)
        .await?;
    Ok(())
}
