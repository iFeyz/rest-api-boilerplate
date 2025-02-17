use actix_web::{web, HttpResponse, post, get, delete, put};
use crate::{
    models::campaign_list::{CampaignList, CreateCampaignListDto, UpdateCampaignListDto, GetCampaignListDto, PaginationDto},
    services::campaign_list_service::CampaignListService,
    error::ApiError,
};
use serde::Deserialize;

pub fn config() -> actix_web::Scope {
    web::scope("/api/campaign_lists")
        .service(create_campaign_list)
        .service(get_campaign_lists)
        .service(update_campaign_list)
        .service(delete_campaign_list)
}

#[derive(Deserialize)]
struct CampaignListQuery {
    campaign_id: i32,
}

#[post("")]
pub async fn create_campaign_list(
    service: web::Data<CampaignListService>,
    campaign_list: web::Json<CreateCampaignListDto>
) -> Result<HttpResponse, ApiError> {
    let campaign_list = service.create_campaign_list(campaign_list.into_inner()).await?;
    Ok(HttpResponse::Created().json(campaign_list))
}

#[get("")]
async fn get_campaign_lists(
    query: web::Query<CampaignListQuery>,
    service: web::Data<CampaignListService>,
) -> HttpResponse {
    match service.get_campaign_lists(query.campaign_id).await {
        Ok(lists) => HttpResponse::Ok().json(lists),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

#[put("/{campaign_id}/{list_id}")]
pub async fn update_campaign_list(
    service: web::Data<CampaignListService>,
    path: web::Path<(i32, i32)>,
    campaign_list: web::Json<UpdateCampaignListDto>
) -> Result<HttpResponse, ApiError> {
    let (campaign_id, list_id) = path.into_inner();
    let campaign_list = service.update_campaign_list(campaign_id, list_id, campaign_list.into_inner()).await?;
    match campaign_list {
        Some(list) => Ok(HttpResponse::Ok().json(list)),
        None => Err(ApiError::NotFound)
    }
}

#[delete("/{campaign_id}/{list_id}")]
pub async fn delete_campaign_list(
    service: web::Data<CampaignListService>,
    path: web::Path<(i32, i32)>
) -> Result<HttpResponse, ApiError> {
    let (campaign_id, list_id) = path.into_inner();
    match service.delete_campaign_list(campaign_id, list_id).await? {
        Some(_) => Ok(HttpResponse::NoContent().finish()),
        None => Err(ApiError::NotFound)
    }
}   