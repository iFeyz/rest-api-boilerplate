use actix_web::{web, HttpResponse, HttpRequest};
use crate::{
    models::subscriber::{Subscriber, CreateSubscriberDto, PaginationParams, SubscriberFilter},
    services::subscriber_service::SubscriberService,
    error::ApiError,
    monitoring::Metrics,
};
use prometheus::IntCounterVec;
use std::sync::Arc;

pub fn config(metrics: Arc<Metrics>) -> actix_web::Scope {
    let counter = IntCounterVec::new(
        prometheus::opts!("api_subscriber_requests_total", "Total number of requests to subscriber endpoints"),
        &["endpoint"]
    ).unwrap();
    
    // Register the counter with the metrics registry, handling potential errors
    if let Err(e) = metrics.registry.register(Box::new(counter.clone())) {
        tracing::warn!("Failed to register subscriber counter: {}", e);
        // Continue even if registration fails
    }
    
    let counter = Arc::new(counter);
    
    web::scope("/subscribers")
        .route("", web::get().to({
            let counter = counter.clone();
            move |_req: HttpRequest, service: web::Data<SubscriberService>, filter: web::Query<SubscriberFilter>, pagination: web::Query<PaginationParams>| {
                counter.with_label_values(&["get_subscribers"]).inc();
                tracing::info!("Getting subscribers with filter: {:?}, pagination: {:?}", filter, pagination);
                async move {
                    match service.get_subscribers(Some(filter.into_inner()), Some(pagination.into_inner())).await {
                        Ok(subscribers) => Ok::<HttpResponse, ApiError>(HttpResponse::Ok().json(subscribers)),
                        Err(e) => Err(e)
                    }
                }
            }
        }))
        .route("/{id_or_email}", web::get().to({
            let counter = counter.clone();
            move |_req: HttpRequest, service: web::Data<SubscriberService>, id_or_email: web::Path<String>| {
                counter.with_label_values(&["get_subscriber"]).inc();
                async move {
                    match service.get_subscriber(id_or_email.into_inner()).await {
                        Ok(subscriber) => Ok::<HttpResponse, ApiError>(HttpResponse::Ok().json(subscriber)),
                        Err(e) => Err(e)
                    }
                }
            }
        }))
        .route("", web::post().to({
            let counter = counter.clone();
            move |_req: HttpRequest, service: web::Data<SubscriberService>, subscriber: web::Json<CreateSubscriberDto>| {
                counter.with_label_values(&["create_subscriber"]).inc();
                async move {
                    match service.create_subscriber(subscriber.into_inner()).await {
                        Ok(subscriber) => Ok::<HttpResponse, ApiError>(HttpResponse::Created().json(subscriber)),
                        Err(e) => Err(e)
                    }
                }
            }
        }))
        .route("/{id_or_email}", web::put().to({
            let counter = counter.clone();
            move |_req: HttpRequest, service: web::Data<SubscriberService>, id_or_email: web::Path<String>, subscriber: web::Json<Subscriber>| {
                counter.with_label_values(&["update_subscriber"]).inc();
                async move {
                    match service.update_subscriber(id_or_email.into_inner(), subscriber.into_inner()).await {
                        Ok(subscriber) => Ok::<HttpResponse, ApiError>(HttpResponse::Ok().json(subscriber)),
                        Err(e) => Err(e)
                    }
                }
            }
        }))
        .route("/{id_or_email}", web::delete().to({
            let counter = counter.clone();
            move |_req: HttpRequest, service: web::Data<SubscriberService>, id_or_email: web::Path<String>| {
                counter.with_label_values(&["delete_subscriber"]).inc();
                async move {
                    match service.delete_subscriber(id_or_email.into_inner()).await {
                        Ok(subscriber) => Ok::<HttpResponse, ApiError>(HttpResponse::Ok().json(subscriber)),
                        Err(e) => Err(e)
                    }
                }
            }
        }))
}


