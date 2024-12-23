use dotenvy::dotenv;
use log::info;
use once_cell::sync::OnceCell;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env;
use std::time::Duration;
use lazy_static::lazy_static;


lazy_static! {
    static ref DB_POOL: OnceCell<Pool<Postgres>> = OnceCell::new();
}

pub fn initialize() {
    dotenv().ok();

    let db_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        env::var("DB_USER").expect("DB_USER is not set"),
        env::var("DB_PASS").expect("DB_PASS is not set"),
        env::var("DB_HOST").expect("DB_HOST is not set"),
        env::var("DB_PORT").unwrap_or_else(|_| "5432".to_string()),
        env::var("DB_NAME").expect("DB_NAME is not set"),
    );

    match PgPoolOptions::new()
        .max_connections(32)
        .min_connections(16)
        .max_lifetime(Duration::from_secs(20_000))
        .idle_timeout(Duration::from_secs(6_000))
        .connect_lazy(&db_url)
    {
        Ok(pool) => {
            DB_POOL.set(pool).expect("Failed to set DB_POOL");
            info!("Database initialized successfully");
        }
        Err(err) => panic!("Failed to create connection pool: {}", err),
    }
}


pub fn get_pool() -> &'static Pool<Postgres> {
    DB_POOL.get().expect("Database pool is not initialized")
}


pub async fn query_task(script: &str) {
    let pool = get_pool();
    sqlx::query(script)
        .execute(pool)
        .await
        .expect("Failed to execute query");
}