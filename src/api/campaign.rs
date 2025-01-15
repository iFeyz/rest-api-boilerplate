use actix_web::{web, HttpResponse, post, get, delete, put};
use crate::{
    models::campaign::{Campaign, CreateCampaignDto, PaginationDto, DeleteCampaignDto, UpdateCampaignDto},
    services::campaign_service::CampaignService,
    error::ApiError,
};

pub fn config() -> actix_web::Scope {
    web::scope("/api/campaigns")
        .service(create_campaign)
        .service(get_campaigns)
        .service(get_campaign)
        .service(update_campaign)
        .service(delete_campaign)
}

#[post("")]
pub async fn create_campaign(
    service: web::Data<CampaignService>,
    campaign: web::Json<CreateCampaignDto>
) -> Result<HttpResponse, ApiError> {
    let campaign = service.create_campaign(campaign.into_inner()).await?;
    Ok(HttpResponse::Created().json(campaign))
}

#[get("/{id}")]
pub async fn get_campaign(
    service: web::Data<CampaignService>,
    id: web::Path<i32>
) -> Result<HttpResponse, ApiError> {
    let campaign = service.get_campaign(id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(campaign))
}

#[get("")]
pub async fn get_campaigns(
    service: web::Data<CampaignService>,
    pagination: web::Query<PaginationDto>
) -> Result<HttpResponse, ApiError> {
    let campaigns = service.get_campaigns(pagination.into_inner()).await?;
    if campaigns.is_none() {
        Ok(HttpResponse::Ok().json(Vec::<Campaign>::new()))
    } else {
        Ok(HttpResponse::Ok().json(campaigns.unwrap()))
    }
}

#[put("/{id}")]
pub async fn update_campaign(
    id: web::Path<i32>,
    campaign: web::Json<UpdateCampaignDto>,
    service: web::Data<CampaignService>,
) -> Result<HttpResponse, ApiError> {
    let mut update_dto = campaign.into_inner();
    update_dto.id = Some(id.into_inner());
    
    let campaign = service.update_campaign(update_dto).await?;
    Ok(HttpResponse::Ok().json(campaign))
}

#[delete("/{id}")]
pub async fn delete_campaign(
    service: web::Data<CampaignService>,
    id: web::Path<i32>
) -> Result<HttpResponse, ApiError> {
    let delete_dto = DeleteCampaignDto {
        id: Some(id.into_inner()),
        uuid: None,
    };
    
    service.delete_campaign(delete_dto).await?;
    Ok(HttpResponse::NoContent().finish())
}
