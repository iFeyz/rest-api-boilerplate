use crate::{
    models::email_views::{
        CreateEmailViewDto,
        PaginationDto,
        GetEmailViewDto,
        EmailView
    },
    services::email_views_service::EmailViewsService,
    error::ApiError,
};

use actix_web::{web, HttpResponse, post};

pub fn config() -> actix_web::Scope {
    web::scope("/api/email_views")
        .service(create_email_view)
}

#[post("")]
pub async fn create_email_view(
    service: web::Data<EmailViewsService>,
    email_view: web::Json<CreateEmailViewDto>
) -> Result<HttpResponse, ApiError> {
    let email_view = service.create_email_view(email_view.into_inner()).await?;
    Ok(HttpResponse::Created().json(email_view))
}