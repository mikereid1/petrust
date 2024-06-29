use actix_web::web;

use crate::service::{
    pet::{create_pet, delete_pet, get_pet, update_pet},
    store::{create_order, delete_order, get_inventory, get_order},
    user::{create_user, delete_user, get_user, login, logout, update_user},
};

pub fn configure_services(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/pet")
            .route("/", web::post().to(create_pet))
            .service(
                web::resource("/{petId}")
                    .route(web::get().to(get_pet))
                    .route(web::put().to(update_pet))
                    .route(web::delete().to(delete_pet)),
            ),
    )
    .service(
        web::scope("/store")
            .route("/inventory", web::post().to(get_inventory))
            .service(
                web::scope("/order")
                    .route("/", web::post().to(create_order))
                    .service(
                        web::resource("/{orderId}")
                            .route(web::get().to(get_order))
                            .route(web::delete().to(delete_order)),
                    ),
            ),
    )
    .service(
        web::scope("/user")
            .route("/", web::post().to(create_user))
            .route("/login", web::post().to(login))
            .route("/logout", web::post().to(logout))
            .service(
                web::resource("/{username}")
                    .route(web::get().to(get_user))
                    .route(web::put().to(update_user))
                    .route(web::delete().to(delete_user)),
            ),
    );
}
