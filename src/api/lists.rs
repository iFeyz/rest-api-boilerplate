use actix_web::{web, HttpResponse, post, get, delete, put};
use crate::{
    models::list::{List, CreateListDto, ListPaginationDto, UpdateListDto},
    services::list_service::ListService,
    error::ApiError,
};


pub fn config() -> actix_web::Scope{
    web::scope("/api/lists")
        .service(create_list)
        .service(get_list_by_id)
        .service(get_lists)
        .service(delete_list)
        .service(update_list)
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