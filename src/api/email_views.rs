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
use std::sync::Arc;
use maxminddb::Reader;
use actix_web::{web, HttpResponse, post , HttpRequest , get};

pub fn config() -> actix_web::Scope {
    web::scope("/api/email_views")
        .service(create_email_view)
        .service(get_email_view)
}

#[post("")]
pub async fn create_email_view(
    service: web::Data<EmailViewsService>,
    email_view: web::Json<CreateEmailViewDto>
) -> Result<HttpResponse, ApiError> {
    let email_view = service.create_email_view(email_view.into_inner()).await?;
    Ok(HttpResponse::Created().json(email_view))
}

#[get("/{subscriber_id}/{sequence_email_id}")]
pub async fn get_email_view(
    path: web::Path<(i32, i32)>,
    service: web::Data<EmailViewsService>,
    req: HttpRequest,
    geoip_reader: web::Data<Arc<Reader<Vec<u8>>>>,
) -> Result<HttpResponse, ApiError> {
    let result = service
        .get_email_view(req, path.into_inner(), geoip_reader.get_ref().as_ref())
        .await?;
    
    Ok(HttpResponse::Ok().json(result))
}