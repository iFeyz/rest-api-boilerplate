use actix_web::{web, HttpResponse, post, get, delete, put};
use crate::{
    models::campaign::{Campaign, CreateCampaignDto},
    services::campaign_service::CampaignService,
    error::ApiError,
};

pub fn config() -> actix_web::Scope {
    web::scope("/api/campaigns")
        .service(create_campaign)
}

#[post("")]
pub async fn create_campaign(
    service: web::Data<CampaignService>,
    campaign: web::Json<CreateCampaignDto>
) -> Result<HttpResponse, ApiError> {
    let campaign = service.create_campaign(campaign.into_inner()).await?;
    Ok(HttpResponse::Created().json(campaign))
}