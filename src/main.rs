mod app_controller;
mod core;
mod domain;
mod features;
mod middlewares;
mod repositories;

use std::sync::Arc;

use actix_web::{App, HttpServer, Result, middleware, web};
use dotenvy::dotenv;
use tracing::{Level, info};

use crate::repositories::user_repository::{InMemoryUserRepo, UserRepo};

#[actix_web::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();
    dotenv().ok();
    let port: i32 = std::env::var("APP_PORT")
        .expect("APP_PORT is required.")
        .parse::<i32>()
        .unwrap();
    let host = format!("0.0.0.0:{}", port);
    let in_memory_repo = InMemoryUserRepo::new();
    let shared_repo: Arc<dyn UserRepo + Send + Sync> = Arc::new(in_memory_repo);
    let app_data = web::Data::from(shared_repo);
    info!("Server is running on http://{}", host);

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .configure(features::router::config)
            .wrap(middleware::Logger::default())
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await?;

    Ok(())
}
