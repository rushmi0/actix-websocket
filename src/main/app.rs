use crate::services::gateway::gateway;
use crate::storage::initialize;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, http, web};
use env_logger::{Env, init_from_env};
use std::path::PathBuf;

pub async fn run() -> std::io::Result<()> {
    init_from_env(Env::default().default_filter_or("info"));
    initialize();

    let static_path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/main/static");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(static_path.clone()))
            .wrap(Logger::default())
            .wrap(cors_config())
            //.route("/{tail:.*}", web::to(gateway))
            .default_service(web::route().to(gateway))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

fn cors_config() -> Cors {
    Cors::default()
        .allow_any_origin()
        .allowed_methods(vec!["GET", "POST"])
        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        .allowed_header(http::header::CONTENT_TYPE)
        .max_age(3700)
        .send_wildcard()
}
