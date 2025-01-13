use actix_web::{web, HttpResponse, post, get , delete , put };
use crate::{
    models::subscriber_list::{SubscriberList, CreateSubscriberListDto, UpdateSubscriberListDto, GetSubscriberListDto},
    services::subscriber_list_service::SubscriberListService,
    error::ApiError,
};

pub fn config() -> actix_web::Scope {
    web::scope("/api/subscriber_lists")
        .service(create_subscriber_list)
        .service(get_subscriber_list)
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


#[get("/{id}_{email}")]
pub async fn get_subscriber_list(
    service: web::Data<SubscriberListService>,
    id: web::Path<i32>
) -> Result<HttpResponse, ApiError> {
    let subscriber_list = service.get_subscriber_list(id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(subscriber_list))
}
