use actix_web::web;

mod index;
mod hello;
mod websocket;

use index::index_service;
use hello::hello_service;
use websocket::websocket_handler;

pub fn service_hub(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(index_service)
            .service(hello_service)
            .route("/ws/event", web::get().to(websocket_handler))
    );
}
