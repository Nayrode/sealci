use std::io::Read;

use crate::app::App;
use crate::event_listener::Listener;
use crate::service::listener::ListenerService;
use actix_multipart::Multipart;
use actix_web::web::Data;
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
    payload: Multipart,
) -> impl Responder {
    HttpResponse::NotImplemented()
}

#[put("/configurations/{id}")]
pub async fn update_configuration(
    data: Data<App>,
    payload: Multipart,
    path: web::Path<usize>,
) -> impl Responder {
    HttpResponse::NotImplemented()
}

#[delete("/configurations/{id}")]
pub async fn delete_configuration(
    listener_service: web::Data<ListenerService>,
    path: web::Path<String>,
) -> impl Responder {
    let id = path.into_inner();
    listener_service.remove_listener(id);
    HttpResponse::Ok().finish()
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

    let mut actions_file_content = String::new();
    match config
        .actions_file
        .as_ref()
        .read_to_string(&mut actions_file_content)
    {
        Ok(_) => (),
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    HttpResponse::Ok().body(actions_file_content)
}
