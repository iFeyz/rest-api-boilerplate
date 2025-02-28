use tokio::time::{sleep, interval};
use std::sync::Arc;
use crate::email_service::{
    models::{EmailRequest, SendStrategy},
    error::EmailError,
    service::EmailService,
};

pub struct EmailExecutor {
    service: Arc<EmailService>,
}

pub struct ExecutorResponse {
    success: bool,
    error: Option<EmailError>,
    message : Option<String>,
}

impl EmailExecutor {
    pub fn new(service: Arc<EmailService>) -> Self {
        Self { service}
    }

    pub async fn execute(&self, request: EmailRequest, strategy: SendStrategy) -> Result<ExecutorResponse, EmailError> {
        match strategy {
            SendStrategy::Immediate => {
                self.service.send_email(request, None).await?;
                Ok(ExecutorResponse {
                    success: true,
                    error: None,
                    message: Some("Email sent successfully immediately".to_string()),
                })
            }


            //TODO: Implémenter les autres stratégies
        }
        //Default case
        Ok(ExecutorResponse {
            success: false,
            error: Some(EmailError::InvalidStrategy),
            message: None,
        })
    }
}

