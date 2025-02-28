use actix_web::{web, HttpResponse, post, get, delete, put, HttpRequest};
use crate::{
    models::subscriber_list::{SubscriberList, CreateSubscriberListDto, UpdateSubscriberListDto, GetSubscriberListDto, PaginationDto},
    services::subscriber_list_service::SubscriberListService,
    error::ApiError,
    monitoring::Metrics,
};
use prometheus::IntCounterVec;
use std::sync::Arc;

pub fn config(metrics: Arc<Metrics>) -> actix_web::Scope {
    let counter = IntCounterVec::new(
        prometheus::opts!("api_subscriber_list_requests_total", "Total number of requests to subscriber list endpoints"),
        &["endpoint"]
    ).unwrap();
    
    // Register the counter with the metrics registry, handling potential errors
    if let Err(e) = metrics.registry.register(Box::new(counter.clone())) {
        tracing::warn!("Failed to register subscriber list counter: {}", e);
        // Continue even if registration fails
    }
    
    let counter = Arc::new(counter);
    
    web::scope("/subscriber_lists")
        .route("", web::get().to({
            let counter = counter.clone();
            move |_req: HttpRequest, service: web::Data<SubscriberListService>, query: web::Query<GetSubscriberListDto>| {
                counter.with_label_values(&["get_subscriber_list"]).inc();
                async move {
                    match service.get_subscriber_list(query.subscriber_id, query.list_id).await {
                        Ok(subscriber_list) => {
                            if subscriber_list.is_none() {
                                Ok::<HttpResponse, ApiError>(HttpResponse::Ok().json(Vec::<SubscriberList>::new()))
                            } else {
                                Ok::<HttpResponse, ApiError>(HttpResponse::Ok().json(subscriber_list.unwrap()))
                            }
                        },
                        Err(e) => Err(e)
                    }
                }
            }
        }))
        .route("/all", web::get().to({
            let counter = counter.clone();
            move |_req: HttpRequest, service: web::Data<SubscriberListService>, query: web::Query<PaginationDto>| {
                counter.with_label_values(&["get_subscriber_lists"]).inc();
                async move {
                    match service.find_all(query.into_inner()).await {
                        Ok(subscriber_lists) => Ok::<HttpResponse, ApiError>(HttpResponse::Ok().json(subscriber_lists)),
                        Err(e) => Err(e)
                    }
                }
            }
        }))
        .route("", web::post().to({
            let counter = counter.clone();
            move |_req: HttpRequest, service: web::Data<SubscriberListService>, subscriber_list: web::Json<CreateSubscriberListDto>| {
                counter.with_label_values(&["create_subscriber_list"]).inc();
                async move {
                    match service.create_subscriber_list(subscriber_list.into_inner()).await {
                        Ok(subscriber_list) => Ok::<HttpResponse, ApiError>(HttpResponse::Created().json(subscriber_list)),
                        Err(e) => Err(e)
                    }
                }
            }
        }))
        .route("/{subscriber_id}/{list_id}", web::put().to({
            let counter = counter.clone();
            move |_req: HttpRequest, service: web::Data<SubscriberListService>, path: web::Path<(i32, i32)>, subscriber_list: web::Json<UpdateSubscriberListDto>| {
                counter.with_label_values(&["update_subscriber_list"]).inc();
                async move {
                    let (subscriber_id, list_id) = path.into_inner();
                    match service.update_subscriber_list(subscriber_id, list_id, subscriber_list.into_inner()).await {
                        Ok(subscriber_list) => {
                            match subscriber_list {
                                Some(list) => Ok::<HttpResponse, ApiError>(HttpResponse::Ok().json(list)),
                                None => Err(ApiError::NotFound)
                            }
                        },
                        Err(e) => Err(e)
                    }
                }
            }
        }))
        .route("/{subscriber_id}/{list_id}", web::delete().to({
            let counter = counter.clone();
            move |_req: HttpRequest, service: web::Data<SubscriberListService>, path: web::Path<(i32, i32)>| {
                counter.with_label_values(&["delete_subscriber_list"]).inc();
                async move {
                    let (subscriber_id, list_id) = path.into_inner();
                    match service.delete_subscriber_list(subscriber_id, list_id).await {
                        Ok(result) => {
                            match result {
                                Some(_) => Ok::<HttpResponse, ApiError>(HttpResponse::NoContent().finish()),
                                None => Err(ApiError::NotFound)
                            }
                        },
                        Err(e) => Err(e)
                    }
                }
            }
        }))
}
