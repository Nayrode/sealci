use crate::application::http::release::handlers::release::{handle_release, list_releases};
use actix_web::web::ServiceConfig;

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(handle_release).service(list_releases);
}
