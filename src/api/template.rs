use actix_web::{web, HttpResponse, post, get , delete , put };
use crate::{
    models::template::{Template, CreateTemplateDto, UpdateTemplateDto, GetTemplateDto, PaginationDto},
    services::template_service::TemplateService,
    error::ApiError,
};

pub fn config() -> actix_web::Scope {
    web::scope("/api/templates")
        .service(create_template)
      //  .service(get_template)
      //  .service(get_templates)
}


#[post("")]
pub async fn create_template(
    service: web::Data<TemplateService>,
    template: web::Json<CreateTemplateDto>
) -> Result<HttpResponse, ApiError> {
    let template = service.create_template(template.into_inner()).await?;
    Ok(HttpResponse::Created().json(template))
}