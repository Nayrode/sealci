use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use tracing::{error, info};

use crate::application::app_context::AppContext;

#[derive(Debug, Deserialize)]
pub struct ReleaseRequest {
    pub repo_url: String,
    pub tag_name: String,
    pub commit_sha: String,
}

#[derive(Debug, Serialize)]
pub struct ReleaseResponse {
    pub status: String,
    pub message: String,
}

#[post("/release")]
pub async fn handle_release(
    release_data: web::Json<ReleaseRequest>,
    ctx: web::Data<AppContext>,
) -> impl Responder {
    info!(
        "Received release request for repo {} with tag {}",
        release_data.repo_url, release_data.tag_name
    );

    // TODO: Implement release handling logic
    // For now, we'll just acknowledge the request
    HttpResponse::Ok().json(ReleaseResponse {
        status: "success".to_string(),
        message: format!(
            "Release request received for tag {} on repository {}",
            release_data.tag_name, release_data.repo_url
        ),
    })
} 