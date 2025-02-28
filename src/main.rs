use actix_web::{web, App, HttpServer};
use actix_web::middleware::{Logger, Compress, DefaultHeaders};
mod middleware;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;
use dotenv::dotenv;
use tracing::{info, error};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use maxminddb::Reader;
use std::sync::Arc;
use crate::middleware::auth::AuthMiddleware;     
use tokio_cron_scheduler::{Job, JobScheduler};
use api_boilerplate::email_service::EmailService;
use api_boilerplate::repositories::sequence_email_repository::SequenceEmailRepository;
use api_boilerplate::services::sequence_email_service::SequenceEmailService;
use api_boilerplate::models::sequence_email::SequenceEmailStatus;
use actix_cors::Cors;
use actix_web_prom::PrometheusMetricsBuilder;
use prometheus::Encoder;
use prometheus::TextEncoder;
use actix_web::HttpResponse;
use api_boilerplate::monitoring::{Metrics, RequestMetricsMiddleware, DatabaseMetrics, EmailMetrics};

use api_boilerplate::{
    repositories::{
        subscriber_repository::SubscriberRepository,
        list_repository::ListsRepository,
        template_repository::TemplateRepository,
        subscriber_list_repository::SubscriberListRepository,
        campaign_repository::CampaignRepository,
        campaign_list_repository::CampaignListRepository,
        email_views_repository::EmailViewsRepository,
        campaign_stats_repository::CampaignStatsRepository,
        global_stats_repository::GlobalStatsRepository,
    },
    services::{
        subscriber_service::SubscriberService,
        list_service::ListService,
        template_service::TemplateService,
        subscriber_list_service::SubscriberListService,
        campaign_service::CampaignService,
        campaign_list_service::CampaignListService,
        email_views_service::EmailViewsService,
        campaign_stats_service::CampaignStatsService,
        global_stats_service::GlobalStatsService,
    },
};

// Import the api module from the crate
use api_boilerplate::api;

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
                        let sequence_id = email.id;
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
                            sequence_id
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

    // Initialiser les métriques
    let metrics = Arc::new(Metrics::new());
    
    // Démarrer le collecteur de métriques système
    Metrics::start_collector(metrics.clone()).await;
    
    // Créer les wrappers de métriques
    let db_metrics = DatabaseMetrics::new(metrics.clone());
    let email_metrics = EmailMetrics::new(metrics.clone());
    
    // Create repositories - some need db_metrics
    let email_views_repository = Arc::new(EmailViewsRepository::new(pool.clone()));
    let subscriber_repository = Arc::new(SubscriberRepository::new(pool.clone()));
    let list_repository = Arc::new(ListsRepository::new(pool.clone(), db_metrics.clone()));
    let template_repository = Arc::new(TemplateRepository::new(pool.clone()));
    let subscriber_list_repository = Arc::new(SubscriberListRepository::new(pool.clone()));
    let campaign_repository = Arc::new(CampaignRepository::new(pool.clone()));
    let campaign_list_repository = Arc::new(CampaignListRepository::new(pool.clone()));
    let sequence_email_repository = Arc::new(SequenceEmailRepository::new(pool.clone()));
    let campaign_stats_repository = Arc::new(CampaignStatsRepository::new(pool.clone()));
    let global_stats_repository = Arc::new(GlobalStatsRepository::new(pool.clone()));

    // Now wrap the pool for web usage
    let pool = web::Data::new(pool);

    // Create services directly without Arc wrapping
    let email_views_service = EmailViewsService::new(EmailViewsRepository::new(pool.get_ref().clone()));
    let subscriber_service = SubscriberService::new(SubscriberRepository::new(pool.get_ref().clone()));
    let list_service = ListService::new(ListsRepository::new(pool.get_ref().clone(), db_metrics.clone()));
    let template_service = TemplateService::new(TemplateRepository::new(pool.get_ref().clone()));
    let subscriber_list_service = SubscriberListService::new(SubscriberListRepository::new(pool.get_ref().clone()));
    let campaign_service = CampaignService::new(CampaignRepository::new(pool.get_ref().clone()));
    let campaign_list_service = CampaignListService::new(CampaignListRepository::new(pool.get_ref().clone()));
    let sequence_email_service = SequenceEmailService::new(SequenceEmailRepository::new(pool.get_ref().clone()));
    let campaign_stats_service = CampaignStatsService::new(web::Data::new(CampaignStatsRepository::new(pool.get_ref().clone())));
    let global_stats_service = GlobalStatsService::new(GlobalStatsRepository::new(pool.get_ref().clone()));

    // Setup email service
    let email_service = setup_email_service().await;

    // Wrap services in web::Data for the HTTP server - only one layer of wrapping
    let email_views_service_data = web::Data::new(email_views_service);
    let subscriber_service_data = web::Data::new(subscriber_service);
    let list_service_data = web::Data::new(list_service);
    let template_service_data = web::Data::new(template_service);
    let subscriber_list_service_data = web::Data::new(subscriber_list_service);
    let campaign_service_data = web::Data::new(campaign_service);
    let campaign_list_service_data = web::Data::new(campaign_list_service);
    let sequence_email_service_data = web::Data::new(sequence_email_service);
    let campaign_stats_service_data = web::Data::new(campaign_stats_service);
    let global_stats_service_data = web::Data::new(global_stats_service);
    let email_service_data = web::Data::new(email_service.clone());

    // Setup GeoIP reader
    let geoip_reader = Reader::open_readfile("GeoIP2-City.mmdb")
        .expect("Failed to load GeoIP database");
    let geoip_reader = Arc::new(geoip_reader);
    let geoip_reader_data = web::Data::new(geoip_reader);

    // Setup the scheduler with wrapped repositories
    if let Err(e) = setup_campaign_scheduler(
        email_service_data.clone(),
        pool.clone(),
        web::Data::new(SequenceEmailRepository::new(pool.get_ref().clone())),
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

    // Configurer le middleware Prometheus avec le registre de métriques
    let prometheus = PrometheusMetricsBuilder::new("api")
        .registry(metrics.registry.clone())
        .endpoint("/prometheus/metrics")
        .build()
        .unwrap();
    
    // Créer le middleware de métriques de requêtes
    let request_metrics = RequestMetricsMiddleware::new(metrics.clone());
    
    // Start the HTTP server
    info!("Starting HTTP server on 0.0.0.0:8081");
    HttpServer::new(move || {
        let registry_clone = metrics.registry.clone();
        let metrics_clone = metrics.clone();
        
        // Create a base app with common middleware (except auth)
        App::new()
            .wrap(Logger::default())
            .wrap(prometheus.clone())
            .wrap(request_metrics.clone())
            .wrap(Compress::default())
            .wrap(DefaultHeaders::new()
                .add(("X-Content-Type-Options", "nosniff"))
                .add(("X-Frame-Options", "DENY"))
                .add(("X-XSS-Protection", "1; mode=block")))
            .wrap(
                Cors::permissive()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600)
                    .expose_headers(vec!["content-type", "authorization"])
            )
            // Register all app_data
            .app_data(pool.clone())
            .app_data(email_views_service_data.clone())
            .app_data(subscriber_service_data.clone())
            .app_data(list_service_data.clone())
            .app_data(template_service_data.clone())
            .app_data(subscriber_list_service_data.clone())
            .app_data(campaign_service_data.clone())
            .app_data(campaign_list_service_data.clone())
            .app_data(email_service_data.clone())
            .app_data(sequence_email_service_data.clone())
            .app_data(campaign_stats_service_data.clone())
            .app_data(geoip_reader_data.clone())
            .app_data(global_stats_service_data.clone())
            .app_data(web::Data::new(db_metrics.clone()))
            .app_data(web::Data::new(email_metrics.clone()))
            
            // Add public routes (metrics endpoints)
            .route("/metrics", web::get().to({
                let registry = registry_clone.clone();
                move || {
                    let registry = registry.clone();
                    async move {
                        let mut buffer = Vec::new();
                        let encoder = TextEncoder::new();
                        let metric_families = registry.gather();
                        encoder.encode(&metric_families, &mut buffer).unwrap();
                        HttpResponse::Ok()
                            .content_type("text/plain; version=0.0.4")
                            .body(buffer)
                    }
                }
            }))
            .route("/prometheus/metrics", web::get().to({
                let registry = registry_clone.clone();
                move || {
                    let registry = registry.clone();
                    async move {
                        let mut buffer = Vec::new();
                        let encoder = TextEncoder::new();
                        let metric_families = registry.gather();
                        encoder.encode(&metric_families, &mut buffer).unwrap();
                        HttpResponse::Ok()
                            .content_type("text/plain; version=0.0.4")
                            .body(buffer)
                    }
                }
            }))
            .route("/health", web::get().to(|| async { HttpResponse::Ok().body("OK") }))
            
            // Add protected routes with auth middleware
            .service(
                web::scope("")
                    .wrap(AuthMiddleware::new("OlH2V4j/OMfBnxfUvsrjoiD9xcI+/ihMv1go8/hf2HI=".to_string()))
                    .route("/test", web::get().to(|| async { HttpResponse::Ok().body("Test route works!") }))
                    .service(
                        web::scope("/api")
                            .configure(|cfg| api::config(cfg, metrics_clone.clone()))
                    )
            )
    })
    .bind(("0.0.0.0", 8081))?
    .run()
    .await
}
