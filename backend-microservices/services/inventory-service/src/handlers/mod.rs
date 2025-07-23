pub mod inventory_handler;

use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(
                web::scope("/products")
                    .route("", web::post().to(inventory_handler::create_product))
                    .route("/{id}", web::get().to(inventory_handler::get_product))
                    .route("/{id}/stock", web::put().to(inventory_handler::update_stock))
                    .route("/{id}/stock", web::get().to(inventory_handler::get_stock))
            )
            .route("/health", web::get().to(inventory_handler::health_check))
    );
} 