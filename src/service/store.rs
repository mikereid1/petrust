use actix_web::{HttpResponse, Responder};

pub async fn get_inventory() -> impl Responder {
    HttpResponse::Ok()
}

pub async fn create_order() -> impl Responder {
    HttpResponse::Ok()
}

pub async fn get_order() -> impl Responder {
    HttpResponse::Ok()
}

pub async fn delete_order() -> impl Responder {
    HttpResponse::Ok()
}