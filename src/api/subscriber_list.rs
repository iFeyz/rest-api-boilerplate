use actix_web::{web, HttpResponse, post, get, delete, put};
use crate::{
    models::subscriber_list::{SubscriberList, CreateSubscriberListDto, UpdateSubscriberListDto, GetSubscriberListDto, PaginationDto},
    services::subscriber_list_service::SubscriberListService,
    error::ApiError,
};

pub fn config() -> actix_web::Scope {
    web::scope("/api/subscriber_lists")
        .service(create_subscriber_list)
        .service(get_subscriber_list)
        .service(get_subscriber_lists)
        .service(update_subscriber_list)
        .service(delete_subscriber_list)
}

#[post("")]
pub async fn create_subscriber_list(
    service: web::Data<SubscriberListService>,
    subscriber_list: web::Json<CreateSubscriberListDto>
) -> Result<HttpResponse, ApiError> {
    let subscriber_list = service.create_subscriber_list(subscriber_list.into_inner()).await?;
    Ok(HttpResponse::Created().json(subscriber_list))
}

#[get("")]
pub async fn get_subscriber_list(
    service: web::Data<SubscriberListService>,
    query: web::Query<GetSubscriberListDto>
) -> Result<HttpResponse, ApiError> {
    let subscriber_list = service.get_subscriber_list(query.subscriber_id, query.list_id).await?;
    if subscriber_list.is_none() {
        Ok(HttpResponse::Ok().json(Vec::<SubscriberList>::new()))
    } else {
        Ok(HttpResponse::Ok().json(subscriber_list.unwrap()))
    }
}

#[get("/all")]
pub async fn get_subscriber_lists(
    service: web::Data<SubscriberListService>,
    query: web::Query<PaginationDto>
) -> Result<HttpResponse, ApiError> {
    let subscriber_lists = service.find_all(query.into_inner()).await?;
    Ok(HttpResponse::Ok().json(subscriber_lists))
}

#[put("/{subscriber_id}/{list_id}")]
pub async fn update_subscriber_list(
    service: web::Data<SubscriberListService>,
    path: web::Path<(i32, i32)>,
    subscriber_list: web::Json<UpdateSubscriberListDto>
) -> Result<HttpResponse, ApiError> {
    let (subscriber_id, list_id) = path.into_inner();
    let subscriber_list = service.update_subscriber_list(subscriber_id, list_id, subscriber_list.into_inner()).await?;
    match subscriber_list {
        Some(list) => Ok(HttpResponse::Ok().json(list)),
        None => Err(ApiError::NotFound)
    }
}

#[delete("/{subscriber_id}/{list_id}")]
pub async fn delete_subscriber_list(
    service: web::Data<SubscriberListService>,
    path: web::Path<(i32, i32)>
) -> Result<HttpResponse, ApiError> {
    let (subscriber_id, list_id) = path.into_inner();
    match service.delete_subscriber_list(subscriber_id, list_id).await? {
        Some(_) => Ok(HttpResponse::NoContent().finish()),
        None => Err(ApiError::NotFound)
    }
}
