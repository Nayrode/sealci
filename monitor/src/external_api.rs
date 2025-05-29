use std::fs::File;
use std::io::{Seek, SeekFrom};
use std::sync::Arc;

use crate::common::{CreateConfigForm, GitEvent, UpdateConfigForm};
use crate::event_listener::Listener;
use crate::service::listener::ListenerService;
use actix_multipart::form::MultipartForm;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

#[get("/configurations")]
pub async fn get_configurations(listener_service: web::Data<ListenerService>) -> impl Responder {
    let listeners = match listener_service.get_all_listeners().await {
        Ok(val) => val,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    let listeners_ref = listeners
        .iter()
        .map(|l| l.as_ref())
        .collect::<Vec<&Listener>>();
    HttpResponse::Ok().json(listeners_ref)
}

#[get("/configurations/{id}")]
pub async fn get_configuration_by_id(
    listener_service: web::Data<ListenerService>,
    path: web::Path<String>,
) -> impl Responder {
    let id = path.into_inner();
    let listener = match listener_service.get_listener(id).await {
        Ok(val) => val,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    HttpResponse::Ok().json(listener.as_ref())
}

#[post("/configurations")]
pub async fn add_configuration(
    listener_service: web::Data<ListenerService>,
    MultipartForm(form): MultipartForm<CreateConfigForm>,
) -> impl Responder {
    // Get a reference to the file
    let file_ref: &File = form.actions_file.file.as_file();

    // Clone the file handle to get an owned File
    let mut action_file = match file_ref.try_clone() {
        Ok(file) => file,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to clone file handle"),
    };

    // Reset the file position to the beginning
    if let Err(_) = action_file.seek(SeekFrom::Start(0)) {
        return HttpResponse::InternalServerError().body("Failed to seek file");
    }

    let action_file = Arc::new(action_file);
    let listener = match listener_service
        .add_listener(
            form.repository_owner.into_inner(),
            form.repository_name.into_inner(),
            action_file,
            form.events.into_inner(),
            form.github_token.into_inner(),
        )
        .await
    {
        Ok(listener) => listener,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    HttpResponse::Created().json(listener.as_ref())
}

#[put("/configurations/{id}")]
pub async fn update_configuration(
    listener_service: web::Data<ListenerService>,
    MultipartForm(form): MultipartForm<UpdateConfigForm>,
    path: web::Path<String>,
) -> impl Responder {
    let id = path.into_inner();
    let mut events: Option<Vec<GitEvent>> = None;
    let mut actions_file: Option<File> = None;

    if let Some(temp_file) = form.actions_file {
        let file_ref: &File = temp_file.file.as_file();

        // Clone the file handle to get an owned File
        let mut cloned_file = match file_ref.try_clone() {
            Ok(file) => file,
            Err(_) => {
                return HttpResponse::InternalServerError().body("Failed to clone file handle")
            }
        };

        // Reset the file position to the beginning
        if let Err(_) = cloned_file.seek(SeekFrom::Start(0)) {
            return HttpResponse::InternalServerError().body("Failed to seek file");
        }

        actions_file = Some(cloned_file);
    }

    if let Some(event_text) = form.events {
        events = Some(event_text.into_inner());
    }

    match listener_service
        .update_listener(id, events, actions_file)
        .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[delete("/configurations/{id}")]
pub async fn delete_configuration(
    listener_service: web::Data<ListenerService>,
    path: web::Path<String>,
) -> impl Responder {
    let id = path.into_inner();
    match listener_service.remove_listener(id).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[get("/configurations/{id}/actions-file")]
pub async fn get_actions_file(
    listener_service: web::Data<ListenerService>,
    path: web::Path<String>,
) -> impl Responder {
    let id = path.into_inner();
    let config = match listener_service.get_listener(id).await {
        Ok(val) => val,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let actions_file_content = match config.action_file_to_string().await {
        Ok(_) => (),
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    HttpResponse::Ok().body(actions_file_content)
}
