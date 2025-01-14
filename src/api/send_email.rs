use actix_web::{web, HttpResponse, post};
use lettre::SmtpTransport;
use crate::email_service::{service::EmailService, models::EmailRequest};
use crate::error::ApiError;

pub fn config() -> actix_web::Scope {
    web::scope("/api/send_email")
        .service(send_email)
}

#[post("")]
pub async fn send_email(
    service: web::Data<EmailService<SmtpTransport>>,
    email: web::Json<EmailRequest>
) -> Result<HttpResponse, ApiError> {
    let response = service.send_email(email.into_inner()).await?;
    Ok(HttpResponse::Ok().json(response))
}
