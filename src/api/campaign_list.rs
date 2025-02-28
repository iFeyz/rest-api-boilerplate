use actix_web::{web, HttpResponse, HttpRequest};
use crate::{
    models::campaign_list::{CreateCampaignListDto, UpdateCampaignListDto},
    services::campaign_list_service::CampaignListService,
    error::ApiError,
    monitoring::Metrics,
};
use serde::Deserialize;
use prometheus::IntCounterVec;
use std::sync::Arc;

#[derive(Deserialize)]
struct CampaignListQuery {
    campaign_id: i32,
}

pub fn config(metrics: Arc<Metrics>) -> actix_web::Scope {
    let counter = IntCounterVec::new(
        prometheus::opts!("api_campaign_list_requests_total", "Total number of requests to campaign list endpoints"),
        &["endpoint"]
    ).unwrap();
    
    // Register the counter with the metrics registry, handling potential errors
    if let Err(e) = metrics.registry.register(Box::new(counter.clone())) {
        tracing::warn!("Failed to register campaign list counter: {}", e);
        // Continue even if registration fails
    }
    
    let counter_arc = Arc::new(counter);
    
    web::scope("/campaign_lists")
        .route("", web::post().to({
            let counter = counter_arc.clone();
            move |_req: HttpRequest, service: web::Data<CampaignListService>, campaign_list: web::Json<CreateCampaignListDto>| {
                counter.with_label_values(&["create_campaign_list"]).inc();
                async move {
                    let campaign_list = service.create_campaign_list(campaign_list.into_inner()).await?;
                    Ok::<HttpResponse, ApiError>(HttpResponse::Created().json(campaign_list))
                }
            }
        }))
        .route("", web::get().to({
            let counter = counter_arc.clone();
            move |_req: HttpRequest, query: web::Query<CampaignListQuery>, service: web::Data<CampaignListService>| {
                counter.with_label_values(&["get_campaign_lists"]).inc();
                async move {
                    match service.get_campaign_lists(query.campaign_id).await {
                        Ok(lists) => Ok::<HttpResponse, ApiError>(HttpResponse::Ok().json(lists)),
                        Err(e) => Ok::<HttpResponse, ApiError>(HttpResponse::InternalServerError().json(e.to_string())),
                    }
                }
            }
        }))
        .route("/{campaign_id}/{list_id}", web::put().to({
            let counter = counter_arc.clone();
            move |_req: HttpRequest, service: web::Data<CampaignListService>, path: web::Path<(i32, i32)>, campaign_list: web::Json<UpdateCampaignListDto>| {
                counter.with_label_values(&["update_campaign_list"]).inc();
                async move {
                    let (campaign_id, list_id) = path.into_inner();
                    let campaign_list = service.update_campaign_list(campaign_id, list_id, campaign_list.into_inner()).await?;
                    match campaign_list {
                        Some(list) => Ok::<HttpResponse, ApiError>(HttpResponse::Ok().json(list)),
                        None => Err(ApiError::NotFound)
                    }
                }
            }
        }))
        .route("/{campaign_id}/{list_id}", web::delete().to({
            let counter = counter_arc.clone();
            move |_req: HttpRequest, service: web::Data<CampaignListService>, path: web::Path<(i32, i32)>| {
                counter.with_label_values(&["delete_campaign_list"]).inc();
                async move {
                    let (campaign_id, list_id) = path.into_inner();
                    match service.delete_campaign_list(campaign_id, list_id).await? {
                        Some(_) => Ok::<HttpResponse, ApiError>(HttpResponse::NoContent().finish()),
                        None => Err(ApiError::NotFound)
                    }
                }
            }
        }))
}   