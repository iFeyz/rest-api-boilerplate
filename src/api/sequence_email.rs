use actix_web::{web, HttpResponse, post, get, delete, put, HttpRequest};
use crate::{
    models::sequence_email::{SequenceEmail, CreateSequenceEmailDto, UpdateSequenceEmailDto, PaginationDto},
    services::sequence_email_service::SequenceEmailService,
    error::ApiError,
    monitoring::Metrics,
};
use std::sync::Arc;
use prometheus::IntCounterVec;

pub fn config(metrics: Arc<Metrics>) -> actix_web::Scope {
    let counter = IntCounterVec::new(
        prometheus::opts!("api_sequence_email_requests_total", "Total number of requests to sequence email endpoints"),
        &["endpoint"]
    ).unwrap();
    
    // Register the counter with the metrics registry, handling potential errors
    if let Err(e) = metrics.registry.register(Box::new(counter.clone())) {
        tracing::warn!("Failed to register sequence email counter: {}", e);
        // Continue even if registration fails
    }
    
    let counter_arc = Arc::new(counter);
    
    web::scope("/sequence-emails")
        .route("", web::post().to({
            let counter = counter_arc.clone();
            move |_req: HttpRequest, service: web::Data<SequenceEmailService>, sequence_email: web::Json<CreateSequenceEmailDto>| {
                counter.with_label_values(&["create_sequence_email"]).inc();
                async move {
                    let sequence_email = service.create_sequence_email(sequence_email.into_inner()).await?;
                    Ok::<HttpResponse, ApiError>(HttpResponse::Created().json(sequence_email))
                }
            }
        }))
        .route("", web::get().to({
            let counter = counter_arc.clone();
            move |req: HttpRequest, service: web::Data<SequenceEmailService>, query: web::Query<PaginationDto>| {
                counter.with_label_values(&["get_sequence_emails"]).inc();
                async move {
                    let pagination = query.into_inner();
                    tracing::debug!("Getting sequence emails with params: campaign_id={}, page={:?}, limit={:?}", 
                        pagination.campaign_id, pagination.page, pagination.limit);
                    
                    let sequence_emails = service.find_all(pagination).await?;
                    Ok::<HttpResponse, ApiError>(HttpResponse::Ok().json(sequence_emails))
                }
            }
        }))
        .route("/campaign/{campaign_id}", web::get().to({
            let counter = counter_arc.clone();
            move |req: HttpRequest, path: web::Path<i32>, service: web::Data<SequenceEmailService>| {
                counter.with_label_values(&["get_sequence_emails_by_campaign"]).inc();
                async move {
                    let campaign_id = path.into_inner();
                    let sequence_emails = service.find_by_campaign_id(campaign_id).await?;
                    Ok::<HttpResponse, ApiError>(HttpResponse::Ok().json(sequence_emails))
                }
            }
        }))
        .route("/{id}", web::delete().to({
            let counter = counter_arc.clone();
            move |_req: HttpRequest, service: web::Data<SequenceEmailService>, id: web::Path<i32>| {
                counter.with_label_values(&["delete_sequence_email"]).inc();
                async move {
                    let sequence_email = service.delete_sequence_email(id.into_inner()).await?;
                    Ok::<HttpResponse, ApiError>(HttpResponse::Ok().json(sequence_email))
                }
            }
        }))
        .route("/{id}", web::put().to({
            let counter = counter_arc.clone();
            move |_req: HttpRequest, service: web::Data<SequenceEmailService>, id: web::Path<i32>, sequence_email: web::Json<UpdateSequenceEmailDto>| {
                counter.with_label_values(&["update_sequence_email"]).inc();
                async move {
                    let sequence_email = service.update_sequence_email(id.into_inner(), sequence_email.into_inner()).await?;
                    Ok::<HttpResponse, ApiError>(HttpResponse::Ok().json(sequence_email))
                }
            }
        }))
        .route("/{id}", web::get().to({
            let counter = counter_arc.clone();
            move |req: HttpRequest, path: web::Path<i32>, service: web::Data<SequenceEmailService>| {
                counter.with_label_values(&["get_sequence_email"]).inc();
                async move {
                    let id = path.into_inner();
                    let sequence_email = service.find_by_id(id).await?;
                    match sequence_email {
                        Some(email) => Ok::<HttpResponse, ApiError>(HttpResponse::Ok().json(email)),
                        None => Err(ApiError::NotFound),
                    }
                }
            }
        }))
}