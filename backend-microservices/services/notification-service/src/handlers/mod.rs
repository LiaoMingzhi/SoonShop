pub mod notification_handler;

use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(
                web::scope("/notifications")
                    .route("", web::get().to(notification_handler::list_notifications))
                    .route("", web::post().to(notification_handler::create_notification))
                    .route("/{id}", web::get().to(notification_handler::get_notification))
                    .route("/{id}", web::put().to(notification_handler::update_notification))
                    .route("/{id}", web::delete().to(notification_handler::delete_notification))
                    .route("/{id}/read", web::post().to(notification_handler::mark_as_read))
                    .route("/{id}/unread", web::post().to(notification_handler::mark_as_unread))
                    .route("/bulk/read", web::post().to(notification_handler::bulk_mark_as_read))
                    .route("/bulk/delete", web::post().to(notification_handler::bulk_delete))
                    .route("/bulk/send", web::post().to(notification_handler::send_bulk_notification))
            )
            .service(
                web::scope("/templates")
                    .route("", web::get().to(notification_handler::list_templates))
                    .route("", web::post().to(notification_handler::create_template))
                    .route("/{id}", web::get().to(notification_handler::get_template))
                    .route("/{id}", web::put().to(notification_handler::update_template))
                    .route("/{id}", web::delete().to(notification_handler::delete_template))
                    .route("/{id}/preview", web::post().to(notification_handler::preview_template))
            )
            .service(
                web::scope("/preferences")
                    .route("", web::get().to(notification_handler::get_user_preferences))
                    .route("", web::put().to(notification_handler::update_user_preferences))
                    .route("/reset", web::post().to(notification_handler::reset_user_preferences))
            )
            .service(
                web::scope("/subscriptions")
                    .route("", web::get().to(notification_handler::list_subscriptions))
                    .route("", web::post().to(notification_handler::subscribe))
                    .route("/{id}", web::delete().to(notification_handler::unsubscribe))
                    .route("/push", web::post().to(notification_handler::subscribe_push))
            )
            .service(
                web::scope("/statistics")
                    .route("", web::get().to(notification_handler::get_statistics))
                    .route("/delivery", web::get().to(notification_handler::get_delivery_statistics))
                    .route("/engagement", web::get().to(notification_handler::get_engagement_statistics))
            )
            .route("/health", web::get().to(notification_handler::health_check))
    );
} 