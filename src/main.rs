use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use env_logger::Env;


mod config;
mod pet;
mod store;
mod user;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(config::configure_services)
    })
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run()
    .await
}
