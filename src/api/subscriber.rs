use actix_web::{web, HttpResponse, post, get , delete , put };
use crate::{
    models::subscriber::{Subscriber, CreateSubscriberDto, GetSubscriberDto , PaginationDto},
    services::subscriber_service::SubscriberService,
    error::ApiError,
};
pub fn config() -> actix_web::Scope {

    web::scope("/api/subscribers")
        .service(create_subscriber)
        .service(get_subscriber)
        .service(get_subscribers)
        .service(delete_subscriber)
        .service(update_subscriber)
      //  .service(get_subscriber_by_email)
      //  .service(get_subscriber_by_id)
      //  .service(delete_subscriber)
}

#[post("")]
pub async fn create_subscriber(
    service: web::Data<SubscriberService>,
    subscriber: web::Json<CreateSubscriberDto>
) -> Result<HttpResponse, ApiError> {
    let subscriber = service.create_subscriber(subscriber.into_inner()).await?;
    Ok(HttpResponse::Created().json(subscriber))
}

#[get("/{id_or_email}")]
pub async fn get_subscriber(
    service: web::Data<SubscriberService>,
    id_or_email: web::Path<String>
) -> Result<HttpResponse, ApiError> {
    let subscriber = service.get_subscriber(id_or_email.into_inner()).await?;
    match subscriber {
        Some(subscriber) => Ok(HttpResponse::Ok().json(subscriber)),
        None => Err(ApiError::NotFound)
    }
}

#[get("")]
pub async fn get_subscribers(
    service: web::Data<SubscriberService>,
    pagination: web::Query<PaginationDto>
) -> Result<HttpResponse, ApiError> {
    let subscribers = service.get_subscribers(pagination.into_inner()).await?;
    if subscribers.is_none() {
        Ok(HttpResponse::Ok().json(Vec::<Subscriber>::new()))
    } else {
        Ok(HttpResponse::Ok().json(subscribers.unwrap()))
    }
}

#[delete("/{id_or_email}")]
pub async fn delete_subscriber(
    service: web::Data<SubscriberService>,
    id_or_email: web::Path<String>
) -> Result<HttpResponse, ApiError> {
    let subscriber = service.delete_subscriber(id_or_email.into_inner()).await?;
    match subscriber {
        Some(subscriber) => Ok(HttpResponse::Ok().json(subscriber)),
        None => Err(ApiError::NotFound)
    }
}

#[put("/{id_or_email}")]
pub async fn update_subscriber(
    service: web::Data<SubscriberService>,
    id_or_email: web::Path<String>,
    subscriber: web::Json<Subscriber>
) -> Result<HttpResponse, ApiError> {
    let subscriber = service.update_subscriber(id_or_email.into_inner(), subscriber.into_inner()).await?;
    match subscriber {
        Some(subscriber) => Ok(HttpResponse::Ok().json(subscriber)),
        None => Err(ApiError::NotFound)
    }
}


