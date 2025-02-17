use crate::email_service::{
    config::{EmailProviderConfig, SmtpConfig},
    providers::{smtp::SmtpProvider, aws_ses::AwsSesProvider, mock::MockProvider},
    service::EmailService,
};

async fn setup_email_service() -> EmailService {
    // Create providers
    let smtp_provider = SmtpProvider::new(SmtpConfig::default())
        .expect("Failed to create SMTP provider");

    // Create the other providers
    let mut service = EmailService::new(smtp_provider);
    service
}

async fn send_email(service: &EmailService, to: String) -> Result<(), EmailError> {
    // Send with default provider
    service.send_email(
        EmailRequest {
            to: to.clone(),
            subject: "Test".to_string(),
            content: "<h1>Hello</h1>".to_string(),
            content_type: ContentType::Html,
            ..Default::default()
        },
        None
    ).await?;

    Ok(())

