use actix_web::{web, HttpResponse, post, get, delete, put, HttpRequest};
use crate::{
    models::list::{List, CreateListDto, ListPaginationDto, UpdateListDto},
    services::list_service::ListService,
    error::ApiError,
    monitoring::Metrics,
};
use prometheus::IntCounterVec;
use std::sync::Arc;

pub fn config(metrics: Arc<Metrics>) -> actix_web::Scope {
    // Create a counter for metrics with a unique name
    let counter = IntCounterVec::new(
        prometheus::opts!("api_lists_requests_total", "Total number of requests to list endpoints"),
        &["endpoint"]
    ).unwrap();
    
    // Register the counter with the metrics registry, handling potential errors
    if let Err(e) = metrics.registry.register(Box::new(counter.clone())) {
        tracing::warn!("Failed to register lists counter: {}", e);
        // Continue even if registration fails
    }
    
    let counter = Arc::new(counter);
    
    web::scope("/lists")
        .route("", web::get().to({
            let counter = counter.clone();
            move |_req: HttpRequest, service: web::Data<ListService>, pagination: web::Query<ListPaginationDto>| {
                let counter = counter.clone();
                async move {
                    // Increment the counter for this endpoint
                    counter.with_label_values(&["get_lists"]).inc();
                    
                    let lists = service.get_lists(pagination.into_inner()).await?;
                    if lists.is_none() {
                        Ok::<HttpResponse, ApiError>(HttpResponse::Ok().json(Vec::<List>::new()))
                    } else {
                        Ok::<HttpResponse, ApiError>(HttpResponse::Ok().json(lists.unwrap()))
                    }
                }
            }
        }))
        .route("/{id}", web::get().to({
            let counter = counter.clone();
            move |_req: HttpRequest, service: web::Data<ListService>, id: web::Path<i32>| {
                let counter = counter.clone();
                async move {
                    // Increment the counter for this endpoint
                    counter.with_label_values(&["get_list_by_id"]).inc();
                    
                    let list = service.get_list_by_id(id.into_inner()).await?;
                    match list {
                        Some(list) => Ok::<HttpResponse, ApiError>(HttpResponse::Ok().json(list)),
                        None => Err(ApiError::NotFound)
                    }
                }
            }
        }))
        .route("/{id}", web::delete().to({
            let counter = counter.clone();
            move |_req: HttpRequest, service: web::Data<ListService>, id: web::Path<i32>| {
                let counter = counter.clone();
                async move {
                    // Increment the counter for this endpoint
                    counter.with_label_values(&["delete_list"]).inc();
                    
                    let list = service.delete_list(id.into_inner()).await?;
                    match list {
                        Some(list) => Ok::<HttpResponse, ApiError>(HttpResponse::Ok().json(list)),
                        None => Err(ApiError::NotFound)
                    }
                }
            }
        }))
        .route("/{id}", web::put().to({
            let counter = counter.clone();
            move |_req: HttpRequest, service: web::Data<ListService>, id: web::Path<i32>, list: web::Json<UpdateListDto>| {
                let counter = counter.clone();
                async move {
                    // Increment the counter for this endpoint
                    counter.with_label_values(&["update_list"]).inc();
                    
                    let mut update_dto = list.into_inner();
                    update_dto.id = id.into_inner();
                    
                    let list = service.update_list(update_dto).await?;
                    Ok::<HttpResponse, ApiError>(HttpResponse::Ok().json(list))
                }
            }
        }))
        .route("", web::post().to({
            let counter = counter.clone();
            move |_req: HttpRequest, service: web::Data<ListService>, list: web::Json<CreateListDto>| {
                let counter = counter.clone();
                async move {
                    // Increment the counter for this endpoint
                    counter.with_label_values(&["create_list"]).inc();
                    
                    let list = service.create_list(list.into_inner()).await?;
                    Ok::<HttpResponse, ApiError>(HttpResponse::Created().json(list))
                }
            }
        }))
}

#[post("")]
pub async fn create_list(
    service: web::Data<ListService>,
    list: web::Json<CreateListDto>
) -> Result<HttpResponse, ApiError> {
    let list = service.create_list(list.into_inner()).await?;
    Ok(HttpResponse::Created().json(list))
}

#[get("/{id}")]
pub async fn get_list_by_id(
    service: web::Data<ListService>,
    id: web::Path<i32>
) -> Result<HttpResponse, ApiError> {
    let list = service.get_list_by_id(id.into_inner()).await?;
    match list {
        Some(list) => Ok(HttpResponse::Ok().json(list)),
        None => Err(ApiError::NotFound)
    }
}

#[get("")]
pub async fn get_lists(
    service: web::Data<ListService>,
    pagination: web::Query<ListPaginationDto>,
) -> Result<HttpResponse, ApiError> {
    let lists = service.get_lists(pagination.into_inner()).await?;
    if lists.is_none() {
        Ok(HttpResponse::Ok().json(Vec::<List>::new()))
    } else {
        Ok(HttpResponse::Ok().json(lists.unwrap()))
    }
}

#[delete("/{id}")]
pub async fn delete_list(
    service: web::Data<ListService>,
    id: web::Path<i32>
) -> Result<HttpResponse, ApiError> {
    let list = service.delete_list(id.into_inner()).await?;
    match list {
        Some(list) => Ok(HttpResponse::Ok().json(list)),
        None => Err(ApiError::NotFound)
    }
}

#[put("/{id}")]
pub async fn update_list(
    service: web::Data<ListService>,
    id: web::Path<i32>,
    list: web::Json<UpdateListDto>
) -> Result<HttpResponse, ApiError> {
    let mut update_dto = list.into_inner();
    update_dto.id = id.into_inner();
    
    let list = service.update_list(update_dto).await?;
    Ok(HttpResponse::Ok().json(list))
}