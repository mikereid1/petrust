use actix_web::{HttpResponse, Responder};

pub async fn create_user() -> impl Responder {
    HttpResponse::Ok()
}

pub async fn login() -> impl Responder {
    HttpResponse::Ok()
}

pub async fn logout() -> impl Responder {
    HttpResponse::Ok()
}

pub async fn get_user() -> impl Responder {
    HttpResponse::Ok()
}

pub async fn update_user() -> impl Responder {
    HttpResponse::Ok()
}

pub async fn delete_user() -> impl Responder {
    HttpResponse::Ok()
}
