pub mod grpc_scheduler_client;
// import proto_scheduler from the generated protobuf code
// The scheduler is a folder containing the .proto files for the scheduler service
pub mod proto_scheduler {
    tonic::include_proto!("scheduler");
}