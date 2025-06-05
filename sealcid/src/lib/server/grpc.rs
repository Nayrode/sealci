use agent::config::Config as AgentConfig;
use tonic::{Request, Response, Status, async_trait};

use crate::{
    common::{
        error::Error,
        mutation::Apply,
        proto::{
            AgentMutation, ControllerMutation, MonitorMutation, ReleaseAgentMutation,
            SchedulerMutation, daemon_server::Daemon as DaemonGrpc,
        },
    },
    server::daemon::Daemon,
};

#[async_trait]
impl DaemonGrpc for Daemon {
    async fn mutate_agent(&self, request: Request<AgentMutation>) -> Result<Response<()>, Status> {
        let mut new_config = request.into_inner();
        let global_config = self.global_config.read().await;
        let mut agent_config: AgentConfig = global_config.to_owned().into();
        new_config.apply(&mut agent_config);
        self.agent
            .restart_with_config(agent_config)
            .await
            .map_err(|e| Status::failed_precondition(Error::RestartAgentError(e)))?;
        Ok(Response::new(()))
    }
    async fn mutate_release_agent(
        &self,
        request: Request<ReleaseAgentMutation>,
    ) -> Result<Response<()>, tonic::Status> {
        todo!()
    }
    async fn mutate_scheduler(
        &self,
        request: Request<SchedulerMutation>,
    ) -> Result<Response<()>, tonic::Status> {
        todo!()
    }

    async fn mutate_monitor(
        &self,
        request: Request<MonitorMutation>,
    ) -> std::result::Result<Response<()>, tonic::Status> {
        todo!()
    }

    async fn mutate_controller(
        &self,
        request: Request<ControllerMutation>,
    ) -> Result<Response<()>, tonic::Status> {
        todo!()
    }
}
