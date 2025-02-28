use actix_web::{web, HttpResponse, HttpRequest};
use crate::services::campaign_stats_service::CampaignStatsService;
use crate::services::global_stats_service::GlobalStatsService;
use std::sync::Arc;
use prometheus::IntCounterVec;
use crate::monitoring::Metrics;

pub fn config(metrics: Arc<Metrics>) -> actix_web::Scope {
    let counter = IntCounterVec::new(
        prometheus::opts!("api_campaign_stats_requests_total", "Total number of requests to campaign stats endpoints"),
        &["endpoint"]
    ).unwrap();
    
    // Register the counter with the metrics registry, handling potential errors
    if let Err(e) = metrics.registry.register(Box::new(counter.clone())) {
        tracing::warn!("Failed to register campaign stats counter: {}", e);
        // Continue even if registration fails
    }
    
    let counter_arc = Arc::new(counter);
    
    web::scope("/stats")
        .route("/global", web::get().to({
            let counter = counter_arc.clone();
            move |_req: HttpRequest, service: web::Data<GlobalStatsService>| {
                counter.with_label_values(&["get_global_stats"]).inc();
                async move {
                    match service.get_global_stats().await {
                        Ok(stats) => HttpResponse::Ok().json(stats),
                        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
                    }
                }
            }
        }))
        .route("/{campaign_id}", web::get().to({
            let counter = counter_arc.clone();
            move |_req: HttpRequest, campaign_id: web::Path<i32>, stats_service: web::Data<CampaignStatsService>| {
                counter.with_label_values(&["get_campaign_stats"]).inc();
                async move {
                    match stats_service.get_campaign_stats(*campaign_id).await {
                        Ok(stats) => HttpResponse::Ok().json(stats),
                        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
                    }
                }
            }
        }))
        .route("/campaign/{campaign_id}/detailed", web::get().to({
            let counter = counter_arc.clone();
            move |_req: HttpRequest, campaign_id: web::Path<i32>, stats_service: web::Data<CampaignStatsService>| {
                counter.with_label_values(&["get_campaign_detailed_stats"]).inc();
                async move {
                    match stats_service.get_campaign_detailed_stats(*campaign_id).await {
                        Ok(stats) => HttpResponse::Ok().json(stats),
                        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
                    }
                }
            }
        }))
        .route("/campaign/{campaign_id}/sequence/{sequence_id}", web::get().to({
            let counter = counter_arc.clone();
            move |_req: HttpRequest, path: web::Path<(i32, i32)>, stats_service: web::Data<CampaignStatsService>| {
                counter.with_label_values(&["get_sequence_email_stats"]).inc();
                async move {
                    let (campaign_id, sequence_id) = path.into_inner();
                    match stats_service.get_sequence_email_stats(campaign_id, sequence_id).await {
                        Ok(stats) => HttpResponse::Ok().json(stats),
                        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
                    }
                }
            }
        }))
}
