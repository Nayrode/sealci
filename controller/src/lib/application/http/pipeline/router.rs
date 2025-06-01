use actix_web::web::ServiceConfig;
use crate::application::http::pipeline::handlers::pipeline::{
    create_pipeline, get_pipelines, get_pipeline,
};

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(get_pipelines)
       .service(get_pipeline)
       .service(create_pipeline);
}