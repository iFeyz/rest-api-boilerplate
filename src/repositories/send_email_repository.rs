use sqlx::PgPool;
use crate::{
    error::ApiError,
    email_service::{
        service::EmailService,
        models::{EmailRequest, EmailResponse},
    },
};
use lettre::transport::smtp::Error as SmtpError;
use lettre::Transport;

#[derive(Clone)]
pub struct SendEmailRepository<T: Transport<Error = SmtpError>> {
    pool: PgPool,
    email_service: EmailService<T>,
}

impl<T: Transport<Error = SmtpError>> SendEmailRepository<T> {
    pub fn new(pool: PgPool, email_service: EmailService<T>) -> Self {
        Self { pool, email_service }
    }

    pub async fn send_email(&self, request: EmailRequest) -> Result<EmailResponse, ApiError> {
        self.email_service.send_email(request).await
            .map_err(|e| ApiError::EmailError(e))
    }
} 