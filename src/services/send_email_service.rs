use crate::{
    repositories::send_email_repository::SendEmailRepository,
    email_service::models::{EmailRequest, EmailResponse},
    error::ApiError,
};

pub struct SendEmailService {
    repository: SendEmailRepository,
}

impl SendEmailService {
    pub fn new(repository: SendEmailRepository) -> Self {
        Self { repository }
    }

    pub async fn send_email(&self, request: EmailRequest) -> Result<EmailResponse, ApiError> {
        self.repository.send_email(request).await
    }
}