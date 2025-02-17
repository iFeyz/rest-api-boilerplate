use actix_web::{web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;
use dotenv::dotenv;
use tracing::{info, warn, error};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use lettre::SmtpTransport;
use maxminddb::Reader;
use std::sync::Arc;
use tokio;
mod api;
mod models;
mod repositories;
mod services;
mod error;
mod email_service;
use std::time::Duration;
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};
use crate::email_service::EmailService;
use crate::repositories::sequence_email_repository::SequenceEmailRepository;
use crate::services::sequence_email_service::SequenceEmailService;
use crate::error::ApiError;
use crate::models::sequence_email::SequenceEmailStatus;

use crate::{
    repositories::{
        subscriber_repository::SubscriberRepository,
        list_repository::ListsRepository,
        template_repository::TemplateRepository,
        subscriber_list_repository::SubscriberListRepository,
        campaign_repository::CampaignRepository,
        campaign_list_repository::CampaignListRepository,
        email_views_repository::EmailViewsRepository,
    },
    services::{
        subscriber_service::SubscriberService,
        list_service::ListService,
        template_service::TemplateService,
        subscriber_list_service::SubscriberListService,
        campaign_service::CampaignService,
        campaign_list_service::CampaignListService,
        email_views_service::EmailViewsService,
    },
};

async fn setup_email_service() -> EmailService {
    let host = std::env::var("SMTP_HOST").expect("SMTP_HOST must be set");
    let username = std::env::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set");
    let password = std::env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set");
    let from_email = std::env::var("FROM_EMAIL").expect("FROM_EMAIL must be set");

    EmailService::new(host, username, password, from_email)
}

async fn setup_campaign_scheduler(
    email_service: web::Data<EmailService>,
    pool: web::Data<PgPool>,
    sequence_email_repository: web::Data<SequenceEmailRepository>,
) -> Result<JobScheduler, Box<dyn std::error::Error>> {
    let scheduler = JobScheduler::new().await?;

    // Check for scheduled campaigns and sequence emails every minute
    scheduler.add(Job::new_async("0 * * * * *", move |_, _| {
        let email_service = email_service.clone();
        let pool = pool.clone();
        let sequence_repo = sequence_email_repository.clone();
        
        Box::pin(async move {
            // Process scheduled campaigns
            if let Err(e) = email_service.process_scheduled_campaigns(&pool).await {
                tracing::error!("Failed to process scheduled campaigns: {}", e);
            }

            // Process pending sequence emails
            match sequence_repo.get_pending_sequence_emails().await {
                Ok(pending_emails) => {
                    for email in pending_emails {
                        // Get campaign lists
                        let lists = match sqlx::query!(
                            r#"
                            SELECT list_id 
                            FROM campaign_lists 
                            WHERE campaign_id = $1
                            "#,
                            email.campaign_id
                        )
                        .fetch_all(&**pool)
                        .await {
                            Ok(lists) => lists,
                            Err(e) => {
                                tracing::error!("Failed to get lists for campaign {}: {}", email.campaign_id, e);
                                continue;
                            }
                        };

                        let list_ids: Vec<i32> = lists.into_iter()
                            .filter_map(|r| r.list_id)
                            .collect();

                        if list_ids.is_empty() {
                            tracing::warn!("No lists found for campaign {}", email.campaign_id);
                            continue;
                        }

                        // Mettre à jour le statut en 'sending'
                        if let Err(e) = sequence_repo.update_status(email.id, SequenceEmailStatus::Sending).await {
                            tracing::error!("Failed to update email status to sending: {}", e);
                            continue;
                        }

                        match email_service.send_emails_to_lists(
                            &pool,
                            &list_ids,
                            &email.subject,
                            &email.body,
                            email.campaign_id,
                            email.id,
                        ).await {
                            Ok(stats) => {
                                // Mettre à jour le statut en 'sent' et désactiver
                                if let Err(e) = sequence_repo.update_status(email.id, SequenceEmailStatus::Sent).await {
                                    tracing::error!("Failed to update email status to sent: {}", e);
                                }
                                
                                // Désactiver l'email
                                match sqlx::query!(
                                    r#"
                                    UPDATE sequence_emails 
                                    SET is_active = false
                                    WHERE id = $1
                                    "#,
                                    email.id
                                )
                                .execute(&**pool)
                                .await {
                                    Ok(_) => {
                                        // Mettre à jour les statistiques de la campagne
                                        let stats_json = match serde_json::to_value(&stats) {
                                            Ok(json) => json,
                                            Err(e) => {
                                                tracing::error!("Failed to serialize stats: {}", e);
                                                continue;
                                            }
                                        };

                                        match sqlx::query!(
                                            r#"
                                            UPDATE campaigns
                                            SET sent = sent + $1,
                                                archive_meta = jsonb_set(
                                                    COALESCE(archive_meta, '{}'::jsonb),
                                                    '{stats}',
                                                    $2::jsonb
                                                )
                                            WHERE id = $3
                                            "#,
                                            stats.successful_sends,
                                            &stats_json,
                                            email.campaign_id
                                        )
                                        .execute(&**pool)
                                        .await {
                                            Ok(_) => tracing::info!("Campaign stats updated successfully"),
                                            Err(e) => tracing::error!("Failed to update campaign stats: {}", e),
                                        }
                                    }
                                    Err(e) => tracing::error!("Failed to deactivate sequence email: {}", e),
                                }
                            }
                            Err(e) => {
                                tracing::error!("Failed to send emails: {}", e);
                                // Mettre à jour le statut en 'failed'
                                if let Err(e) = sequence_repo.update_status(email.id, SequenceEmailStatus::Failed).await {
                                    tracing::error!("Failed to update email status to failed: {}", e);
                                }
                            }
                        }
                    }
                }
                Err(e) => tracing::error!("Failed to get pending sequence emails: {}", e),
            }
        })
    })?).await?;

    scheduler.start().await?;
    Ok(scheduler)
}

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

    // Create the database pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    info!("Database connection established");

    // Run migrations before wrapping pool in web::Data
    info!("Running database migrations...");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate database");
    info!("Database migrations completed");

    // Create repositories with the unwrapped pool
    let email_views_repository = EmailViewsRepository::new(pool.clone());
    let subscriber_repository = SubscriberRepository::new(pool.clone());
    let list_repository = ListsRepository::new(pool.clone());
    let template_repository = TemplateRepository::new(pool.clone());
    let subscriber_list_repository = SubscriberListRepository::new(pool.clone());
    let campaign_repository = CampaignRepository::new(pool.clone());
    let campaign_list_repository = CampaignListRepository::new(pool.clone());
    let sequence_email_repository = SequenceEmailRepository::new(pool.clone());

    // Now wrap the pool for web usage
    let pool = web::Data::new(pool);

    // Create services and wrap repositories in web::Data
    let email_views_service = EmailViewsService::new(email_views_repository.clone());
    let email_views_service = web::Data::new(email_views_service);
    let email_views_repository = web::Data::new(email_views_repository);
    let subscriber_service = web::Data::new(SubscriberService::new(subscriber_repository));
    let list_service = web::Data::new(ListService::new(list_repository));
    let template_service = web::Data::new(TemplateService::new(template_repository));
    let subscriber_list_service = web::Data::new(SubscriberListService::new(subscriber_list_repository));
    let campaign_service = web::Data::new(CampaignService::new(campaign_repository));
    let campaign_list_service = web::Data::new(CampaignListService::new(campaign_list_repository));
    let sequence_email_repository = web::Data::new(sequence_email_repository.clone());
    let sequence_email_service = web::Data::new(SequenceEmailService::new(sequence_email_repository.get_ref().clone()));

    // Setup other services
    let email_service = setup_email_service().await;
    let email_service = web::Data::new(email_service);

    let geoip_reader = Reader::open_readfile("GeoIP2-City.mmdb")
        .expect("Failed to load GeoIP database");
    let geoip_reader = Arc::new(geoip_reader);
    let geoip_reader = web::Data::new(geoip_reader);

    // Setup the scheduler with wrapped repositories
    if let Err(e) = setup_campaign_scheduler(
        email_service.clone(),
        pool.clone(),
        sequence_email_repository.clone(),
    ).await {
        error!("Failed to setup scheduler: {}", e);
    }

    // Add this after setting up email_service in main()
    info!("Testing email service...");
    if let Err(e) = email_service.send_email(
        "your.test@email.com",
        "Test Email",
        "This is a test email"
    ).await {
        error!("Failed to send test email: {}", e);
    } else {
        info!("Test email sent successfully");
    }

    // Start the HTTP server
    info!("Starting HTTP server on 127.0.0.1:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .app_data(email_views_service.clone())
            .app_data(subscriber_service.clone())
            .app_data(list_service.clone())
            .app_data(template_service.clone())
            .app_data(subscriber_list_service.clone())
            .app_data(campaign_service.clone())
            .app_data(campaign_list_service.clone())
            .app_data(email_service.clone())
            .app_data(sequence_email_service.clone())
            .app_data(geoip_reader.clone())
            .service(api::subscriber::config())
            .service(api::lists::config())
            .service(api::template::config())
            .service(api::subscriber_list::config())
            .service(api::campaign::config())
            .service(api::campaign_list::config())
            .service(api::sequence_email::config())
            .service(api::email_views::config())
            .configure(api::send_email::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
