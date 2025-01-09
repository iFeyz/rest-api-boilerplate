use actix_web::{middleware , web , App , HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::{layer::SubscriberExt , util::SubscriberInitExt};

use crate::repositories::user_repository::UserRepository;
use crate::services::user_service::UserService;

mod api;
mod config;
mod error;
mod models;
mod repositories;
mod services;

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
    let user_repository = UserRepository::new(pool.clone());
    let user_service = web::Data::new(UserService::new(user_repository));

    //Creation du serveur HTTP
    println!("Server running on http://{}:{}", config.server.host, config.server.port);
    HttpServer::new(move || {
        App::new()
            .app_data(user_service.clone())
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::trim())
            .configure(api::config)
    })
    .bind((config.server.host, config.server.port))?
    .run()
    .await
}
