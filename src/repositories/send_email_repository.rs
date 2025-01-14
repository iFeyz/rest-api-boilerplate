use sqlx::PgPool;
use lettre::transport::smtp::Error as SmtpError;
use lettre::{Transport, SmtpTransport};
use crate::{
    error::ApiError,
    email_service::{
        service::EmailService,
        models::{EmailRequest, EmailResponse},
    },
};

pub struct SendEmailRepository {
    pool: PgPool,
    email_service: EmailService<SmtpTransport>,
}

impl SendEmailRepository {
    pub fn new(pool: PgPool, email_service: EmailService<SmtpTransport>) -> Self {
        Self { pool, email_service }
    }

    pub async fn send_email(&self, request: EmailRequest) -> Result<EmailResponse, ApiError> {
        let response = self.email_service.send_email(request).await
            .map_err(|e| ApiError::EmailError(e))?;
        Ok(response)
    }
} 