use actix_web::{web, HttpResponse, HttpRequest};
use crate::{
    models::template::{CreateTemplateDto, UpdateTemplateDto, PaginationDto},
    services::template_service::TemplateService,
    error::ApiError,
    monitoring::Metrics,
};
use prometheus::IntCounterVec;
use std::sync::Arc;

pub fn config(metrics: Arc<Metrics>) -> actix_web::Scope {
    let counter = IntCounterVec::new(
        prometheus::opts!("api_template_requests_total", "Total number of requests to template endpoints"),
        &["endpoint"]
    ).unwrap();
    
    // Register the counter with the metrics registry, handling potential errors
    if let Err(e) = metrics.registry.register(Box::new(counter.clone())) {
        tracing::warn!("Failed to register template counter: {}", e);
        // Continue even if registration fails
    }
    
    let counter = Arc::new(counter);
    
    web::scope("/templates")
        .route("", web::get().to({
            let counter = counter.clone();
            move |_req: HttpRequest, service: web::Data<TemplateService>, query: web::Query<PaginationDto>| {
                counter.with_label_values(&["get_templates"]).inc();
                async move {
                    match service.find_all(query.into_inner()).await {
                        Ok(templates) => Ok::<HttpResponse, ApiError>(HttpResponse::Ok().json(templates)),
                        Err(e) => Err(e)
                    }
                }
            }
        }))
        .route("/{id}", web::get().to({
            let counter = counter.clone();
            move |_req: HttpRequest, service: web::Data<TemplateService>, path: web::Path<i32>| {
                counter.with_label_values(&["get_template_by_id"]).inc();
                async move {
                    match service.find_by_id(path.into_inner()).await {
                        Ok(template) => Ok::<HttpResponse, ApiError>(HttpResponse::Ok().json(template)),
                        Err(e) => Err(e)
                    }
                }
            }
        }))
        .route("", web::post().to({
            let counter = counter.clone();
            move |_req: HttpRequest, service: web::Data<TemplateService>, template: web::Json<CreateTemplateDto>| {
                counter.with_label_values(&["create_template"]).inc();
                async move {
                    match service.create(template.into_inner()).await {
                        Ok(template) => Ok::<HttpResponse, ApiError>(HttpResponse::Created().json(template)),
                        Err(e) => Err(e)
                    }
                }
            }
        }))
        .route("/{id}", web::put().to({
            let counter = counter.clone();
            move |_req: HttpRequest, service: web::Data<TemplateService>, path: web::Path<i32>, template: web::Json<UpdateTemplateDto>| {
                counter.with_label_values(&["update_template"]).inc();
                async move {
                    match service.update(path.into_inner(), template.into_inner()).await {
                        Ok(template) => Ok::<HttpResponse, ApiError>(HttpResponse::Ok().json(template)),
                        Err(e) => Err(e)
                    }
                }
            }
        }))
        .route("/{id}", web::delete().to({
            let counter = counter.clone();
            move |_req: HttpRequest, service: web::Data<TemplateService>, path: web::Path<i32>| {
                counter.with_label_values(&["delete_template"]).inc();
                async move {
                    match service.delete(path.into_inner()).await {
                        Ok(template) => Ok::<HttpResponse, ApiError>(HttpResponse::Ok().json(template)),
                        Err(e) => Err(e)
                    }
                }
            }
        }))
}