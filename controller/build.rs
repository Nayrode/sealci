fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build the protobuf files for the scheduler service
    tonic_build::configure()
        .build_server(true)
        .compile(
            &["../api/proto/scheduler/controller.proto"],
            &["../api/proto/scheduler"],
        )
        .expect("Building scheduler protobuf failed");

    tonic_build::configure()
        .build_server(false)
        .compile(
            &["../api/proto/release-agent/controller.proto"],
            &["../api/proto/release-agent"],
        )
        .expect("Building scheduler protobuf failed");
    Ok(())
}
