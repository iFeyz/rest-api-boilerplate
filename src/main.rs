use actix_web::{web, App, HttpServer};
use actix_web::middleware::{Logger, NormalizePath};
use actix_cors::Cors;
use actix_web::http::header;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::repositories::subscriber_repository::SubscriberRepository;
use crate::services::subscriber_service::SubscriberService;
use crate::repositories::lists_repository::ListsRepository;
use crate::services::list_service::ListService;
use crate::middleware::auth::AuthMiddleware;
use crate::repositories::template_repository::TemplateRepository;
use crate::services::template_service::TemplateService;
use crate::repositories::subscriber_list_repository::SubscriberListRepository;
use crate::services::subscriber_list_service::SubscriberListService;
use crate::repositories::campaign_repository::CampaignRepository;
use crate::services::campaign_service::CampaignService;

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
    
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or("info".to_string())
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = config::Config::from_env().expect("Server configuration failed");
    let api_key = std::env::var("API_KEY").expect("API_KEY is not set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("Failed to connect to the database");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate the database");

    let subscriber_repository = SubscriberRepository::new(pool.clone());
    let subscriber_service = web::Data::new(SubscriberService::new(subscriber_repository));

    let lists_repository = ListsRepository::new(pool.clone());
    let lists_service = web::Data::new(ListService::new(lists_repository));

    let template_repository = TemplateRepository::new(pool.clone());
    let template_service = web::Data::new(TemplateService::new(template_repository));

    let subscriber_list_repository = SubscriberListRepository::new(pool.clone());
    let subscriber_list_service = web::Data::new(SubscriberListService::new(subscriber_list_repository));

    let campaign_repository = CampaignRepository::new(pool.clone());
    let campaign_service = web::Data::new(CampaignService::new(campaign_repository));

    println!("Server running on http://{}:{}", config.server.host, config.server.port);
    HttpServer::new(move || {
        let cors = Cors::permissive()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::ACCEPT,
                header::AUTHORIZATION,
                header::HeaderName::from_static("x-api-key"),
            ])
            .expose_headers(vec![
                header::CONTENT_TYPE,
                header::ACCEPT,
                header::AUTHORIZATION,
                header::HeaderName::from_static("x-api-key"),
            ])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(NormalizePath::trim())
            .wrap(AuthMiddleware::new(api_key.clone()))
            .app_data(subscriber_service.clone())
            .app_data(lists_service.clone())
            .app_data(template_service.clone())
            .app_data(subscriber_list_service.clone())
            .app_data(campaign_service.clone())
            .configure(api::config)
    })
    .bind((config.server.host, config.server.port))?
    .run()
    .await
}
