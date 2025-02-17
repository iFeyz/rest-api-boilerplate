use actix_web::{get, web, HttpResponse};
use crate::services::campaign_stats_service::CampaignStatsService;

pub fn config() -> actix_web::Scope {
    web::scope("/api/stats")
        .service(get_campaign_stats)
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
