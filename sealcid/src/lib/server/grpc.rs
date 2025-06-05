use tonic::async_trait;

use crate::{
    common::proto::proto::{
        AgentMutation, ControllerMutation, MonitorMutation, MutationResponse, ReleaseAgentMutation,
        SchedulerMutation, daemon_server::Daemon as DaemonGrpc,
    },
    server::daemon::{self, Daemon},
};

#[async_trait]
impl DaemonGrpc for Daemon {
    async fn mutate_agent(
        &self,
        request: tonic::Request<AgentMutation>,
    ) -> Result<tonic::Response<MutationResponse>, tonic::Status> {
        todo!()
    }
    async fn mutate_release_agent(
        &self,
        request: tonic::Request<ReleaseAgentMutation>,
    ) -> Result<tonic::Response<MutationResponse>, tonic::Status> {
        todo!()
    }
    async fn mutate_scheduler(
        &self,
        request: tonic::Request<SchedulerMutation>,
    ) -> Result<tonic::Response<MutationResponse>, tonic::Status> {
        todo!()
    }

    async fn mutate_monitor(
        &self,
        request: tonic::Request<MonitorMutation>,
    ) -> std::result::Result<tonic::Response<MutationResponse>, tonic::Status> {
        todo!()
    }

    async fn mutate_controller(
        &self,
        request: tonic::Request<ControllerMutation>,
    ) -> Result<tonic::Response<MutationResponse>, tonic::Status> {
        todo!()
    }
}
