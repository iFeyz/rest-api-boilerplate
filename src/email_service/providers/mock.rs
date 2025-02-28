use async_trait::async_trait;
use crate::email_service::{
    provider::EmailProvider,
    models::EmailRequest,
    error::EmailError,
    config::AwsSesConfig
};

#[async_trait]
impl EmailProvider for MockProvider {
    async fn send(&self , request: EmailRequest) -> Result<String , EmailError> { 
        if self.should_fail {
            Err(EmailError::ProviderError("Mock failure".to_string()))
        } else {
            Ok(format!("mock_message_id_{}", Uuid::new_v4()))
        }
    }
    

    fn provider_name(&self) -> &'static str {
        "mock"
    }

}