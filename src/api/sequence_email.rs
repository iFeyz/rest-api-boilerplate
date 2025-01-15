use actix_web::{web, HttpResponse, post, get, delete, put};
use crate::{
    models::sequence_emails::{SequenceEmail, CreateSequenceEmailDto, UpdateSequenceEmailDto, DeleteSequenceEmailDto, PaginationDto},
    services::sequence_emails_service::SequenceEmailService,
    error::ApiError,
};

pub fn config() -> actix_web::Scope {
    web::scope("/api/sequence-emails")
        .service(create_sequence_email)
        .service(get_sequence_emails)
        .service(delete_sequence_email)
        .service(update_sequence_email)
}

#[post("")]
pub async fn create_sequence_email(
    service: web::Data<SequenceEmailService>,
    sequence_email: web::Json<CreateSequenceEmailDto>
) -> Result<HttpResponse, ApiError> {
    let sequence_email = service.create_sequence_email(sequence_email.into_inner()).await?;
    Ok(HttpResponse::Created().json(sequence_email))
}


#[get("")]
pub async fn get_sequence_emails(
    service: web::Data<SequenceEmailService>,
    query: web::Query<PaginationDto>
) -> Result<HttpResponse, ApiError> {
    let sequence_emails = service.find_all(query.into_inner()).await?;
    Ok(HttpResponse::Ok().json(sequence_emails))
}


#[delete("/{id}")]
pub async fn delete_sequence_email(
    service: web::Data<SequenceEmailService>,
    id: web::Path<i32>
) -> Result<HttpResponse, ApiError> {
    let sequence_email = service.delete_sequence_email(id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(sequence_email))
}

#[put("/{id}")]
pub async fn update_sequence_email(
    service: web::Data<SequenceEmailService>,
    id: web::Path<i32>,
    sequence_email: web::Json<UpdateSequenceEmailDto>
) -> Result<HttpResponse, ApiError> {
    let sequence_email = service.update_sequence_email(id.into_inner(), sequence_email.into_inner()).await?;
    Ok(HttpResponse::Ok().json(sequence_email))
}