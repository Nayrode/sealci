use crate::application::http::release::handlers::release::{
    handle_release, list_releases, pks_lookup,
};
use actix_web::web::ServiceConfig;

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(handle_release)
        .service(list_releases)
        .service(pks_lookup);
}
