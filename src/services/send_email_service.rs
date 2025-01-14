use crate::{
    error::ApiError,
    repositories::send_email_repository::SendEmailRepository,
    email_service::models::{EmailRequest, EmailResponse},
};
use lettre::transport::smtp::Error as SmtpError;
use lettre::Transport;

#[derive(Clone)]
pub struct SendEmailService<T: Transport<Error = SmtpError>> {
    repository: SendEmailRepository<T>,
}

impl<T: Transport<Error = SmtpError>> SendEmailService<T> {
    pub fn new(repository: SendEmailRepository<T>) -> Self {
        Self { repository }
    }

    pub async fn send_email(&self, request: EmailRequest) -> Result<EmailResponse, ApiError> {
        self.repository.send_email(request).await
    }
}