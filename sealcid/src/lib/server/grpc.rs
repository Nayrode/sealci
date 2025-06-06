use agent::config::Config as AgentConfig;
use tonic::{Request, Response, Status, async_trait};
use monitor::{config::Config as MonitorConfig};
use controller::{config::Config as ControllerConfig};
use sealci_scheduler::config::Config as SchedulerConfig;

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
use crate::server::config::{GlobalConfig, Update};

#[async_trait]
impl DaemonGrpc for Daemon {
    async fn mutate_agent(&self, request: Request<AgentMutation>) -> Result<Response<()>, Status> {
        let mut new_config = request.into_inner();
        let mut writer = self.global_config.write().await;
        writer.update(agent_config);
        let global_config = self.global_config.read().await;
        let mut agent_config: AgentConfig = global_config.to_owned().into();
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
        let mut new_config = request.into_inner();
        let mut writer = self.global_config.write().await;
        writer.update(new_config);
        let global_config = self.global_config.read().await;
        let mut scheduler_config: SchedulerConfig = global_config.to_owned().into();
        self.scheduler
            .restart_with_config(scheduler_config)
            .await
            .map_err(|e| Status::failed_precondition(Error::RestartSchedulerError(e)))?;
        let controller_config: ControllerConfig = global_config.to_owned().into();
        self.controller
            .restart_with_config(controller_config)
            .await
            .map_err(|e| Status::failed_precondition(Error::RestartControllerError(e)))?;
        let agent_config: AgentConfig = global_config.to_owned().into();
        self.agent
            .restart_with_config(agent_config)
            .await
            .map_err(|e| Status::failed_precondition(Error::RestartAgentError(e)))?;
        Ok(Response::new(()))
    }

    async fn mutate_monitor(
        &self,
        request: Request<MonitorMutation>,
    ) -> std::result::Result<Response<()>, tonic::Status> {
        let mut new_config = request.into_inner();
        let mut writer = self.global_config.write().await;
        writer.update(new_config);
        let global_config = self.global_config.read().await;
        let mut monitor_config: MonitorConfig = global_config.to_owned().into();
        self.monitor
            .restart_with_config(monitor_config)
            .await
            .map_err(|e| Status::failed_precondition(Error::RestartMonitorError(e)))?;
        Ok(Response::new(()))
    }

    async fn mutate_controller(
        &self,
        request: Request<ControllerMutation>,
    ) -> Result<Response<()>, tonic::Status> {
        let mut new_config = request.into_inner();
        let mut writer = self.global_config.write().await;
        writer.update(controller_config);
        let global_config = self.global_config.read().await;
        let mut controller_config: ControllerConfig = global_config.to_owned().into();
        self.controller
            .restart_with_config(controller_config)
            .await
            .map_err(|e| Status::failed_precondition(Error::RestartControllerError(e)))?;
        let monitor_config: MonitorConfig = global_config.to_owned().into();
        self.monitor.restart_with_config(monitor_config).await.map_err(|e| Status::failed_precondition(Error::RestartMonitorError(e)))?;
        Ok(Response::new(()))
    }
}
