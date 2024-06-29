mod service;

use actix_web::{App, HttpServer};
use service::config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new().configure(config::configure_services)
    })
        .bind(("127.0.0.1", 8080))?
        .workers(2)
        .run()
        .await
}