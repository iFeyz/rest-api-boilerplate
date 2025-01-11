use actix_web::{web, App, HttpServer};
use actix_web::middleware::{Logger, NormalizePath};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::repositories::subscriber_repository::SubscriberRepository;
use crate::services::subscriber_service::SubscriberService;
use crate::repositories::lists_repository::ListsRepository;
use crate::services::list_service::ListService;
use crate::middleware::auth::AuthMiddleware;

mod api;
mod config;
mod error;
mod models;
mod repositories;
mod services;
mod middleware;

pub struct AppState {
    db : sqlx::PgPool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    //Config des logs en rust
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or("info".to_string())
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
    // Chargement de la configuration
    let config = config::Config::from_env().expect("Server configuration failed");
    let api_key = std::env::var("API_KEY").expect("API_KEY is not set");
    // Creation de la pool de connexion a la base de donnees
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("Failed to connect to the database");

    // Après la création du pool
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate the database");

    // Initialisation des composants
    let subscriber_repository = SubscriberRepository::new(pool.clone());
    let subscriber_service = web::Data::new(SubscriberService::new(subscriber_repository));

    let lists_repository = ListsRepository::new(pool.clone());
    let lists_service = web::Data::new(ListService::new(lists_repository));

    //Creation du serveur HTTP
    println!("Server running on http://{}:{}", config.server.host, config.server.port);
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(NormalizePath::trim())
            .wrap(AuthMiddleware::new(api_key.clone()))
            .app_data(subscriber_service.clone())
            .app_data(lists_service.clone())
            .configure(api::config)
    })
    .bind((config.server.host, config.server.port))?
    .run()
    .await
}
