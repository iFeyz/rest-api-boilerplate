use actix_web::{web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;
use std::env;
use dotenv::dotenv;
use tracing::{info, warn, error};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use lettre::SmtpTransport;

mod api;
mod models;
mod repositories;
mod services;
mod error;
mod email_service;

use crate::{
    repositories::{
        subscriber_repository::SubscriberRepository,
        list_repository::ListsRepository,
        template_repository::TemplateRepository,
        subscriber_list_repository::SubscriberListRepository,
        campaign_repository::CampaignRepository,
        campaign_list_repository::CampaignListRepository,
        send_email_repository::SendEmailRepository,
        sequence_email_repository::SequenceEmailRepository,
        email_views_repository::EmailViewsRepository,
    },
    services::{
        subscriber_service::SubscriberService,
        list_service::ListService,
        template_service::TemplateService,
        subscriber_list_service::SubscriberListService,
        campaign_service::CampaignService,
        campaign_list_service::CampaignListService,
        send_email_service::SendEmailService,
        sequence_emails_service::SequenceEmailService,
        email_views_service::EmailViewsService,
    },
    email_service::{EmailService, config::SmtpConfig},
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Initialize logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into())
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Starting email service application...");

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    info!("Database connection established");

    info!("Running database migrations...");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate database");
    info!("Database migrations completed");

    let subscriber_repository = SubscriberRepository::new(pool.clone());
    let subscriber_service = web::Data::new(SubscriberService::new(subscriber_repository));

    let list_repository = ListsRepository::new(pool.clone());
    let list_service = web::Data::new(ListService::new(list_repository));

    let template_repository = TemplateRepository::new(pool.clone());
    let template_service = web::Data::new(TemplateService::new(template_repository));

    let subscriber_list_repository = SubscriberListRepository::new(pool.clone());
    let subscriber_list_service = web::Data::new(SubscriberListService::new(subscriber_list_repository));

    let sequence_email_repository = SequenceEmailRepository::new(pool.clone());
    let sequence_email_service = web::Data::new(SequenceEmailService::new(sequence_email_repository));

    let campaign_repository = CampaignRepository::new(pool.clone());
    let campaign_service = web::Data::new(CampaignService::new(campaign_repository));

    let campaign_list_repository = CampaignListRepository::new(pool.clone());
    let campaign_list_service = web::Data::new(CampaignListService::new(campaign_list_repository));

    let email_views_repository = EmailViewsRepository::new(pool.clone());
    let email_views_service = web::Data::new(EmailViewsService::new(email_views_repository));

    info!("Initializing email service with SMTP config...");
    let email_service: EmailService<SmtpTransport> = EmailService::with_config(SmtpConfig::default())
        .expect("Failed to create email service");
    let send_email_repository = SendEmailRepository::new(pool.clone(), email_service);
    let send_email_service = web::Data::new(SendEmailService::new(send_email_repository));

    info!("Starting HTTP server on 127.0.0.1:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(subscriber_service.clone())
            .app_data(list_service.clone())
            .app_data(template_service.clone())
            .app_data(subscriber_list_service.clone())
            .app_data(campaign_service.clone())
            .app_data(campaign_list_service.clone())
            .app_data(send_email_service.clone())
            .app_data(sequence_email_service.clone())
            .app_data(email_views_service.clone())
            .service(api::subscriber::config())
            .service(api::lists::config())
            .service(api::template::config())
            .service(api::subscriber_list::config())
            .service(api::campaign::config())
            .service(api::campaign_list::config())
            .service(api::send_email::config())
            .service(api::sequence_email::config())
            .service(api::email_views::config())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
