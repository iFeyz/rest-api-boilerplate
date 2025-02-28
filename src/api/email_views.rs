use crate::{
    models::email_views::{
        CreateEmailViewDto,
        PaginationDto,
        GetEmailViewDto,
        EmailView
    },
    services::email_views_service::EmailViewsService,
    error::ApiError,
    monitoring::Metrics,
};
use std::sync::Arc;
use maxminddb::Reader;
use actix_web::{web, HttpResponse, HttpRequest};
use prometheus::IntCounterVec;

pub fn config(metrics: Arc<Metrics>) -> actix_web::Scope {
    let counter = IntCounterVec::new(
        prometheus::opts!("api_email_views_requests_total", "Total number of requests to email views endpoints"),
        &["endpoint"]
    ).unwrap();
    
    // Register the counter with the metrics registry, handling potential errors
    if let Err(e) = metrics.registry.register(Box::new(counter.clone())) {
        tracing::warn!("Failed to register email views counter: {}", e);
        // Continue even if registration fails
    }
    
    let counter_arc = Arc::new(counter);
    
    web::scope("/email-views")
        .route("", web::post().to({
            let counter = counter_arc.clone();
            move |_req: HttpRequest, service: web::Data<EmailViewsService>, email_view: web::Json<CreateEmailViewDto>| {
                counter.with_label_values(&["create_email_view"]).inc();
                async move {
                    let email_view = service.create_email_view(email_view.into_inner()).await?;
                    Ok::<HttpResponse, ApiError>(HttpResponse::Created().json(email_view))
                }
            }
        }))
        .route("/{subscriber_id}/{sequence_email_id}/{campaign_id}", web::get().to({
            let counter = counter_arc.clone();
            move |req: HttpRequest, path: web::Path<(i32, i32, i32)>, service: web::Data<EmailViewsService>, geoip_reader: web::Data<Arc<Reader<Vec<u8>>>>| {
                counter.with_label_values(&["get_email_view"]).inc();
                async move {
                    tracing::info!("Received tracking request for: {:?}", path);
                    
                    let result = service
                        .get_email_view(req, path.into_inner(), geoip_reader.get_ref().as_ref())
                        .await?;
                    
                    Ok::<HttpResponse, ApiError>(HttpResponse::Ok().json(result))
                }
            }
        }))
}