use actix_web::{body::BoxBody, get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{
    application::{app_context::AppContext, ports::release_service::ReleaseService},
    domain::releases::entities::{Release, ReleaseError},
};

#[derive(Deserialize)]
struct ListReleasesQuery {
    owner: String,
    repo: String,
}

#[derive(Deserialize)]
struct PksLookup {
    search: String,
    op: String,
    options: String,
}

#[derive(Debug, Deserialize)]
pub struct ReleaseRequest {
    pub repo_url: String,
    pub tag_name: String,
}

#[derive(Debug, Serialize)]
pub struct ReleaseResponse {
    pub status: String,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct ReleasesResponse {
    pub status: String,
    pub releases: Vec<Release>,
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

    let release_service = ctx.release_service.clone();
    match release_service
        .create_release(
            release_data.repo_url.to_string().as_str(),
            release_data.tag_name.to_string().as_str(),
        )
        .await
    {
        Ok(_) => HttpResponse::Ok().json(ReleaseResponse {
            status: "success".to_string(),
            message: format!(
                "Release request received for tag {} on repository {}",
                release_data.tag_name, release_data.repo_url
            ),
        }),
        Err(ReleaseError::DatabaseError(e)) => {
            HttpResponse::InternalServerError().json(ReleaseResponse {
                status: "error".to_string(),
                message: e.to_string(),
            })
        }
        Err(ReleaseError::NotFound) => HttpResponse::NotFound().json(ReleaseResponse {
            status: "error".to_string(),
            message: "Release not found".to_string(),
        }),
        Err(ReleaseError::InternalError) => {
            HttpResponse::InternalServerError().json(ReleaseResponse {
                status: "error".to_string(),
                message: "Internal server error :(".to_string(),
            })
        }
        Err(ReleaseError::ReleaseAgentError) => {
            HttpResponse::InternalServerError().json(ReleaseResponse {
                status: "error".to_string(),
                message: "Release agent error :(".to_string(),
            })
        }
    }

    // TODO: Implement release handling logic
    // For now, we'll just acknowledge the request
}

#[get("/release/{owner}/{repo}")]
pub async fn list_releases(
    path: web::Path<ListReleasesQuery>,
    ctx: web::Data<AppContext>,
) -> impl Responder {
    let release_service = ctx.release_service.clone();
    let releases = release_service
        .list_releases(format!("https://github.com/{}/{}", path.owner, path.repo).as_str())
        .await;
    match releases {
        Err(_) => HttpResponse::InternalServerError().json(ReleasesResponse {
            releases: vec![],
            status: "error".to_string(),
        }),
        Ok(list_releases) => HttpResponse::Ok().json(ReleasesResponse {
            releases: list_releases,
            status: "success".to_string(),
        }),
    }
}

#[get("/pks/lookup")]
pub async fn pks_lookup(
    ctx: web::Data<AppContext>,
    query: web::Query<PksLookup>,
) -> impl Responder {
    let operation = query.op.clone();
    let fingerprint = query.search.clone();
    if operation != "get" {
        return HttpResponse::BadRequest().json(ReleaseResponse {
            message: format!("Operation {} not implemented", operation),
            status: "error".to_string(),
        });
    }
    let fingerprint = match fingerprint.split("0x").last() {
        Some(fp) => fp.to_string(),
        None => {
            return HttpResponse::BadRequest().json(ReleaseResponse {
                message: "Invalid fingerprint format. Expected '0x' prefix.".to_string(),
                status: "error".to_string(),
            });
        }
    };
    let key_result = ctx.release_service.get_key(&fingerprint.clone()).await;
    match key_result {
        Err(_) => HttpResponse::NotFound().json(ReleaseResponse {
            message: format!("Key with fingerprint {} not found", fingerprint),
            status: "error".to_string(),
        }),
        Ok(key) => HttpResponse::Ok()
            .append_header(("Content-Type", "text/plain"))
            .body(BoxBody::new(key)),
    }
}
