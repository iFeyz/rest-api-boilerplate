use actix_web::{get, web, HttpResponse};
use crate::services::campaign_stats_service::CampaignStatsService;
use crate::services::global_stats_service::GlobalStatsService;

pub fn config() -> actix_web::Scope {
    web::scope("/api/stats")
        .service(get_global_stats)
        .service(get_campaign_stats)
        .service(get_campaign_detailed_stats)
        .service(get_sequence_email_stats)
}

#[get("/global")]
async fn get_global_stats(service: web::Data<GlobalStatsService>) -> HttpResponse {
    match service.get_global_stats().await {
        Ok(stats) => HttpResponse::Ok().json(stats),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

#[get("/{campaign_id}")]
async fn get_campaign_stats(
    campaign_id: web::Path<i32>,
    stats_service: web::Data<CampaignStatsService>,
) -> HttpResponse {
    match stats_service.get_campaign_stats(*campaign_id).await {
        Ok(stats) => HttpResponse::Ok().json(stats),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

#[get("/campaign/{campaign_id}/detailed")]
async fn get_campaign_detailed_stats(
    campaign_id: web::Path<i32>,
    stats_service: web::Data<CampaignStatsService>,
) -> HttpResponse {
    match stats_service.get_campaign_detailed_stats(*campaign_id).await {
        Ok(stats) => HttpResponse::Ok().json(stats),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

#[get("/campaign/{campaign_id}/sequence/{sequence_id}")]
async fn get_sequence_email_stats(
    path: web::Path<(i32, i32)>,
    stats_service: web::Data<CampaignStatsService>,
) -> HttpResponse {
    let (campaign_id, sequence_id) = path.into_inner();
    match stats_service.get_sequence_email_stats(campaign_id, sequence_id).await {
        Ok(stats) => HttpResponse::Ok().json(stats),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
