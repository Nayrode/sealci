use crate::errors::Error;

use crate::proto::actions as proto;
use proto::action_service_client::ActionServiceClient as ActionClient;

use crate::logic::action_queue_logic::Action;

use tonic::transport::Channel;
use tonic::Request;
use tracing::debug;

pub(crate) async fn execution_action(
    action: Action, agent_address: String
) -> Result<tonic::Streaming<proto::ActionResponseStream>, Error> {
    debug!("Received action: {:?}", action);

    debug!("Attempting to connect to agent at address: {}", agent_address);

    let channel = Channel::builder(
        agent_address
            .parse::<http::uri::Uri>()
            .map_err(|e| Error::GrpcClientError(tonic::Status::internal(e.to_string())))?
    )
    .connect()
    .await
    .map_err(|e| Error::GrpcClientError(tonic::Status::internal(e.to_string())))?;
    let mut client = ActionClient::new(channel);

    debug!("Creating ActionRequest for action ID: {}", action.get_action_id());

    let request = Request::new(proto::ActionRequest {
        action_id: action.get_action_id(),
        context: Some(proto::ExecutionContext {
            r#type: action.get_runner_type(),
            container_image: Some(String::from(action.get_container_image())),
        }),
        commands: action.get_commands().iter().map(|comm: &String| String::from(comm)).collect(),
        repo_url: action.get_repo_url().clone(),
    });

    debug!("Sending ActionRequest: {:?}", request);

    // The response stream is returned to the caller function for further processing. (controller_interface.rs)
    let response_stream = client.execution_action(request).await
        .map_err(|e| Error::GrpcClientError(tonic::Status::internal(e.to_string())))?.into_inner();
    Ok(response_stream)
}
