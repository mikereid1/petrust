use actix_web::{HttpResponse, Responder};

pub async fn create_pet() -> impl Responder {
    HttpResponse::Ok()
}

pub async fn get_pet() -> impl Responder {
    HttpResponse::Ok()
}

pub async fn update_pet() -> impl Responder {
    HttpResponse::Ok()
}

pub async fn delete_pet() -> impl Responder {
    HttpResponse::Ok()
}