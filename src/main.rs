use std::env;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use diesel::{Connection, PgConnection};
use dotenvy::dotenv;
use env_logger::Env;


mod config;
mod pet;
mod store;
mod user;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

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
