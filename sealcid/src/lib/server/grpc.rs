use agent::config::Config as AgentConfig;
use controller::config::Config as ControllerConfig;
use monitor::config::Config as MonitorConfig;
use sealci_scheduler::config::Config as SchedulerConfig;
use tonic::{Request, Response, Status, async_trait};

use crate::server::config::Update;
use crate::{
    common::{
        error::Error,
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
        let new_config = request.into_inner();
        if Some(false) == new_config.toggle_agent {
            self.agent.disable().await.map_err(|e| {
                Status::failed_precondition(Error::RestartAgentError(e))
            })?;
            return Ok(Response::new(()));
        }
        self.global_config.write().await.update(new_config);
        let global_config = self.global_config.read().await;
        let agent_config: AgentConfig = global_config.to_owned().into();
        self.agent
            .restart_with_config(agent_config.clone())
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
        let new_config = request.into_inner();
        if Some(false) == new_config.toggle_scheduler {
            self.scheduler.disable().await.map_err(|e| {
                Status::failed_precondition(Error::RestartSchedulerError(e))
            })?;
            return Ok(Response::new(()));
        }
        self.global_config.write().await.update(new_config);
        let global_config = self.global_config.read().await;
        let scheduler_config: SchedulerConfig = global_config.to_owned().into();
        self.scheduler
            .restart_with_config(scheduler_config)
            .await
            .map_err(|e| Status::failed_precondition(Error::RestartSchedulerError(e)))?;
        let controller_config: ControllerConfig = global_config.to_owned().into();
        self.controller
            .restart_with_config(controller_config)
            .await
            .map_err(|e| {
                Status::failed_precondition(Error::RestartControllerError(e))
            })?;
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
        let new_config = request.into_inner();
        if Some(false) == new_config.toggle_monitor {
            self.monitor.disable().await.map_err(|e| {
                Status::failed_precondition(Error::RestartMonitorError(e))
            })?;
            return Ok(Response::new(()));
        }
        self.global_config.write().await.update(new_config);
        let monitor_config: MonitorConfig = self.global_config.read().await.to_owned().into();
        self.monitor
            .restart_with_config(monitor_config.clone())
            .await
            .map_err(|e| Status::failed_precondition(Error::RestartMonitorError(e)))?;
        Ok(Response::new(()))
    }

    async fn mutate_controller(
        &self,
        request: Request<ControllerMutation>,
    ) -> Result<Response<()>, tonic::Status> {
        let new_config = request.into_inner();
        if Some(false) == new_config.toggle_controller {
            self.controller.disable().await.map_err(|e| {
                Status::failed_precondition(Error::RestartControllerError(e))
            })?;
            return Ok(Response::new(()));
        }
        self.global_config.write().await.update(new_config);
        let global_config = self.global_config.read().await;
        let controller_config: ControllerConfig = global_config.to_owned().into();
        self.controller
            .restart_with_config(controller_config)
            .await
            .map_err(|e| {
                Status::failed_precondition(Error::RestartControllerError(e))
            })?;
        let monitor_config: MonitorConfig = global_config.to_owned().into();
        self.monitor
            .restart_with_config(monitor_config)
            .await
            .map_err(|e| Status::failed_precondition(Error::RestartMonitorError(e)))?;
        Ok(Response::new(()))
    }
}
