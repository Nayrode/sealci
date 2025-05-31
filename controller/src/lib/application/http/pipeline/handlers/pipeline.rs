use actix_multipart::form::{tempfile::TempFile, text::Text as MpText, MultipartForm};
use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Deserialize;
use std::io::Read;
use std::sync::Arc;
use tracing::{error, info};

use crate::application::ports::pipeline_service::PipelineService;
use crate::application::services::pipeline_service::DefaultPipelineServiceImpl;
use crate::parser::pipe_parser::{
    ManifestParser, ManifestPipeline as ParserManifestPipeline, ParsingError, PipeParser,
};

use crate::domain::pipeline::entities::pipeline::{
    ActionManifest as DomainActionManifest, ActionsMap, Configuration,
    ManifestPipeline as DomainManifestPipeline, PipelineError,
};

#[derive(Debug, MultipartForm)]
struct UploadPipelineForm {
    #[multipart(rename = "body")]
    file: TempFile,
    repo_url: MpText<String>,
}

#[derive(Deserialize)]
struct PipelineByIDQuery {
    id: i64,
}

#[derive(Deserialize)]
struct PipelineQueryParams {
    verbose: Option<bool>,
}

#[get("/pipeline")]
pub async fn get_pipelines(
    pipeline_service: web::Data<Arc<DefaultPipelineServiceImpl>>,
    query: web::Query<PipelineQueryParams>,
) -> impl Responder {
    let verbose = query.verbose.unwrap_or(false);
    match pipeline_service.find_all(verbose).await {
        Ok(pipelines) => HttpResponse::Ok().json(pipelines),
        Err(e) => {
            error!("Error fetching pipelines: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/pipeline/{id}")]
pub async fn get_pipeline(
    path: web::Path<PipelineByIDQuery>,
    pipeline_service: web::Data<Arc<DefaultPipelineServiceImpl>>,
    query: web::Query<PipelineQueryParams>,
) -> impl Responder {
    let id = path.id;
    let verbose = query.verbose.unwrap_or(false);
    info!("Fetching pipeline with id: {}, verbose: {}", id, verbose);
    match pipeline_service.find_by_id(id).await {
        Ok(mut pipeline) => {
            if verbose {
                if let Err(e) = pipeline_service.add_verbose_details(&mut pipeline).await {
                    error!("Failed to enrich pipeline {} with logs: {:?}", id, e);
                }
            }
            HttpResponse::Ok().json(pipeline)
        }

        Err(PipelineError::NotFound) => HttpResponse::NotFound().finish(),

        Err(e) => {
            error!("Error fetching pipeline {}: {:?}", id, e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/pipeline")]
pub async fn create_pipeline(
    MultipartForm(form): MultipartForm<UploadPipelineForm>,
    pipeline_service: web::Data<Arc<DefaultPipelineServiceImpl>>,
) -> impl Responder {
    info!(
        "Uploaded file {} with repository {}",
        form.file.size,
        form.repo_url.as_str()
    );
    let repo_url = form.repo_url.to_string();
    let mut fd = form.file.file;
    let mut buffer = String::new();
    if let Err(e) = fd.read_to_string(&mut buffer) {
        if e.kind() == std::io::ErrorKind::InvalidData {
            return HttpResponse::UnprocessableEntity().body("Invalid data");
        }
        return HttpResponse::InternalServerError().finish();
    }
    let parser = PipeParser {};
    let parser_manifest: ParserManifestPipeline = match parser.parse(buffer) {
        Ok(m) => m,
        Err(ParsingError::YamlNotCompliant) => {
            return HttpResponse::BadRequest().body("Invalid YAML format");
        }
        Err(e) => return HttpResponse::BadRequest().body(format!("Parse error: {:?}", e)),
    };

    let actions_map = parser_manifest
        .actions
        .into_iter()
        .map(|action| {
            let domain_action = DomainActionManifest {
                commands: action.commands,
                configuration: Configuration {
                    container: action.configuration_version,
                },
            };
            (action.name, domain_action)
        })
        .collect();
    let domain_manifest = DomainManifestPipeline {
        name: parser_manifest.name,
        actions: ActionsMap {
            actions: actions_map,
        },
    };

    match pipeline_service
        .create_manifest_pipeline(domain_manifest, repo_url)
        .await
    {
        Ok(p) => HttpResponse::Ok().json(p),
        Err(e) => {
            error!("create_manifest_pipeline failed: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
