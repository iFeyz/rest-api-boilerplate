use actix_web::{web, HttpResponse, post, get, delete, put};
use crate::{
    models::template::{Template, CreateTemplateDto, UpdateTemplateDto, GetTemplateDto, PaginationDto},
    services::template_service::TemplateService,
    error::ApiError,
};

pub fn config() -> actix_web::Scope {
    web::scope("/api/templates")
        .service(create_template)
        .service(get_templates)
        .service(get_template_by_id)
        .service(update_template)
        .service(delete_template)
}

#[post("")]
pub async fn create_template(
    service: web::Data<TemplateService>,
    template: web::Json<CreateTemplateDto>
) -> Result<HttpResponse, ApiError> {
    let template = service.create(template.into_inner()).await?;
    Ok(HttpResponse::Created().json(template))
}

#[get("/{id}")]
pub async fn get_template_by_id(
    service: web::Data<TemplateService>,
    path: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let template = service.find_by_id(path.into_inner()).await?;
    Ok(HttpResponse::Ok().json(template))
}

#[get("")]
pub async fn get_templates(
    service: web::Data<TemplateService>,
    query: web::Query<PaginationDto>,
) -> Result<HttpResponse, ApiError> {
    let templates = service.find_all(query.into_inner()).await?;
    Ok(HttpResponse::Ok().json(templates))
}

#[put("/{id}")]
pub async fn update_template(
    service: web::Data<TemplateService>,
    path: web::Path<i32>,
    template: web::Json<UpdateTemplateDto>,
) -> Result<HttpResponse, ApiError> {
    let template = service.update(path.into_inner(), template.into_inner()).await?;
    Ok(HttpResponse::Ok().json(template))
}

#[delete("/{id}")]
pub async fn delete_template(
    service: web::Data<TemplateService>,
    path: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    service.delete(path.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}