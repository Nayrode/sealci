// this is a mock server to avoid getting errors

use std::thread;

use std::time::Duration;

use grpc_scheduler::{controller_server::Controller, ActionRequest, ActionResponse};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{async_trait, Request, Response, Status};

pub mod grpc_scheduler {
    tonic::include_proto!("scheduler");
}

#[derive(Debug)]
pub struct MockSchedulerService {}

#[async_trait]
impl Controller for MockSchedulerService {
    type ScheduleActionStream = ReceiverStream<Result<ActionResponse, Status>>;

    async fn schedule_action(
        &self,
        _request: Request<ActionRequest>,
    ) -> Result<Response<Self::ScheduleActionStream>, Status> {
        let (tx, rx) = mpsc::channel(10);

        println!(
            "{}",
            _request
                .get_ref()
                .context
                .as_ref()
                .unwrap()
                .container_image
                .as_ref()
                .unwrap()
        );

        for _i in 0..10 {
            println!("INFO: scheduled");
            tx.send(Ok(ActionResponse {
                action_id: _request.get_ref().action_id,
                log: "INFO: scheduled".to_string(),
                result: Some(grpc_scheduler::ActionResult {
                    completion: grpc_scheduler::ActionStatus::Scheduled as i32,
                    exit_code: Some(1),
                }),
            }))
            .await
            .expect("should be sent");

            thread::sleep(Duration::from_millis(500));
        }

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}
