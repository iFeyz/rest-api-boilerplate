use actix_web::{web, HttpResponse, post, get, delete, put, HttpRequest};
use crate::{
    models::campaign::{Campaign, CreateCampaignDto, DeleteCampaignDto, UpdateCampaignDto, CampaignParams, PaginationParams, CampaignFilter},
    services::campaign_service::CampaignService,
    error::ApiError,
    monitoring::Metrics,
};
use std::sync::Arc;
use prometheus::IntCounterVec;

pub fn config(metrics: Arc<Metrics>) -> actix_web::Scope {
    let counter = IntCounterVec::new(
        prometheus::opts!("api_campaign_requests_total", "Total number of requests to campaign endpoints"),
        &["endpoint"]
    ).unwrap();
    
    // Register the counter with the metrics registry, handling potential errors
    if let Err(e) = metrics.registry.register(Box::new(counter.clone())) {
        tracing::warn!("Failed to register campaign counter: {}", e);
        // Continue even if registration fails
    }
    
    let counter_arc = Arc::new(counter);
    
    web::scope("/campaigns")
        .service(web::resource("")
            .route(web::get().to({
                let counter = counter_arc.clone();
                move |_req: HttpRequest, service: web::Data<CampaignService>, filter: web::Query<CampaignFilter>, pagination: web::Query<PaginationParams>| {
                    counter.with_label_values(&["get_campaigns"]).inc();
                    async move {
                        let campaigns = service.get_campaigns(Some(filter.into_inner()), Some(pagination.into_inner())).await?;
                        Ok::<HttpResponse, ApiError>(HttpResponse::Ok().json(campaigns))
                    }
                }
            }))
            .route(web::post().to({
                let counter = counter_arc.clone();
                move |_req: HttpRequest, service: web::Data<CampaignService>, campaign: web::Json<CreateCampaignDto>| {
                    counter.with_label_values(&["create_campaign"]).inc();
                    async move {
                        let campaign = service.create_campaign(campaign.into_inner()).await?;
                        Ok::<HttpResponse, ApiError>(HttpResponse::Created().json(campaign))
                    }
                }
            }))
        )
        .service(web::resource("/{id}")
            .route(web::get().to({
                let counter = counter_arc.clone();
                move |_req: HttpRequest, service: web::Data<CampaignService>, id: web::Path<i32>| {
                    counter.with_label_values(&["get_campaign"]).inc();
                    async move {
                        let campaign = service.get_campaign(id.into_inner()).await?;
                        Ok::<HttpResponse, ApiError>(HttpResponse::Ok().json(campaign))
                    }
                }
            }))
            .route(web::put().to({
                let counter = counter_arc.clone();
                move |_req: HttpRequest, id: web::Path<i32>, campaign: web::Json<UpdateCampaignDto>, service: web::Data<CampaignService>| {
                    counter.with_label_values(&["update_campaign"]).inc();
                    async move {
                        let mut update_dto = campaign.into_inner();
                        update_dto.id = Some(id.into_inner());
                        
                        let campaign = service.update_campaign(update_dto).await?;
                        Ok::<HttpResponse, ApiError>(HttpResponse::Ok().json(campaign))
                    }
                }
            }))
            .route(web::delete().to({
                let counter = counter_arc.clone();
                move |_req: HttpRequest, service: web::Data<CampaignService>, id: web::Path<i32>| {
                    counter.with_label_values(&["delete_campaign"]).inc();
                    async move {
                        let delete_dto = DeleteCampaignDto {
                            id: Some(id.into_inner()),
                            uuid: None,
                        };
                        
                        service.delete_campaign(delete_dto).await?;
                        Ok::<HttpResponse, ApiError>(HttpResponse::NoContent().finish())
                    }
                }
            }))
        )
}
