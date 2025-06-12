use actix_web::web::ServiceConfig;
use crate::application::http::release::handlers::release::handle_release;

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(handle_release);
} 