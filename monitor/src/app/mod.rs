use std::sync::Arc;

use actix_web::{web::Data, App as ActixApp, HttpServer};
use tracing::info;

use crate::{
    config::Config,
    error::Error,
    external_api::{
        add_configuration, delete_configuration, doc, get_actions_file, get_configuration_by_id,
        get_configurations, update_configuration,
    },
    github::GitHubClient,
    service::listener::ListenerService,
};

pub struct App {
    config: Config,
    listener_service: Arc<ListenerService>,
}

impl App {
    pub fn new(config: Config) -> Self {
        let github_client = Arc::new(GitHubClient::new());
        let controller_client = Arc::new(crate::controller::ControllerClient::new(
            config.controller_host.clone(),
        ));
        let listener_service = Arc::new(ListenerService::new(
            github_client.clone(),
            controller_client.clone(),
        ));
        App {
            config,
            listener_service,
        }
    }

    pub async fn run(&self) -> Result<(), Error> {
        // Initialize the application, set up routes, etc.
        info!("Application is running...");
        let listener_service = self.listener_service.clone();
        HttpServer::new({
            let listener_service = listener_service.clone();
            move || {
                ActixApp::new()
                    .app_data(Data::new(listener_service.clone()))
                    .service(get_configurations)
                    .service(get_configuration_by_id)
                    .service(add_configuration)
                    .service(update_configuration)
                    .service(delete_configuration)
                    .service(get_actions_file)
                    .service(doc)
            }
        })
        .bind(("0.0.0.0", self.config.port))
        .map_err(Error::ServerError)?
        .run()
        .await
        .map_err(Error::ServerError)?;
        Ok(())
    }
}
