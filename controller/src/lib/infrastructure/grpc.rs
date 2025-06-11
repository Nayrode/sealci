pub mod grpc_scheduler_client;
pub mod grpc_release_agent_client;
// import proto_scheduler from the generated protobuf code
// The scheduler is a folder containing the .proto files for the scheduler service
pub mod proto_scheduler {
    tonic::include_proto!("scheduler");
}

pub mod proto_release_agent {
    tonic::include_proto!("releaseagent");
}
