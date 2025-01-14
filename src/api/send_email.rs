use actix_web::{web, HttpResponse, post};
use chrono::Utc;
use crate::{
    email_service::models::{EmailRequest, EmailResponse},
    services::{send_email_service::SendEmailService, campaign_service::CampaignService},
    error::ApiError,
    models::campaign::{UpdateCampaignDto, CampaignStatus},
    repositories::campaign_repository::CampaignRepository,
};
use lettre::SmtpTransport;
use tracing::{info, error};
use serde::{Deserialize, Serialize};
use tokio::time::{sleep, Duration};
use std::time::Instant;
use sqlx::PgPool;
use serde_json::json;

const MAX_RETRIES: u32 = 3;
const RETRY_DELAY_MS: u64 = 1000;
const RATE_LIMIT_DELAY_MS: u64 = 100;

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailRequestDto {
    pub to: String,
    pub subject: String,
    pub content: String,
    pub campaign_id: String,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct BulkEmailResponse {
    pub successes: Vec<EmailResponse>,
    pub failures: Vec<String>,
    pub total: usize,
    pub success_count: usize,
    pub duration_secs: f64,
}

pub fn config() -> actix_web::Scope {
    web::scope("/api/send_email")
        .service(send_email)
        .service(send_bulk_email)
}

async fn send_with_retry(
    service: &web::Data<SendEmailService<SmtpTransport>>,
    request: EmailRequest,
    retries: u32
) -> Result<EmailResponse, ApiError> {
    let mut attempt = 0;

    while attempt < retries {
        match service.send_email(request.clone()).await {
            Ok(response) => return Ok(response),
            Err(e) => {
                attempt += 1;
                error!("Attempt {}/{} failed for {}: {}", attempt, retries, request.to, e);
                if attempt < retries {
                    sleep(Duration::from_millis(RETRY_DELAY_MS)).await;
                } else {
                    return Err(e.into());
                }
            }
        }
    }

    unreachable!()
}

#[post("")]
pub async fn send_email(
    service: web::Data<SendEmailService<SmtpTransport>>,
    email: web::Json<EmailRequestDto>
) -> Result<HttpResponse, ApiError> {
    info!("Received email request to: {}", email.to);
    
    let request = EmailRequest {
        to: email.to.clone(),
        subject: email.subject.clone(),
        content: email.content.clone(),
        metadata: Default::default(),
    };
    
    let response = send_with_retry(&service, request, MAX_RETRIES).await?;
    info!("Email sent successfully");
    Ok(HttpResponse::Ok().json(response))
}

#[post("/bulk")]
pub async fn send_bulk_email(
    service: web::Data<SendEmailService<SmtpTransport>>,
    pool: web::Data<PgPool>,
    emails: web::Json<Vec<EmailRequestDto>>
) -> Result<HttpResponse, ApiError> {
    let start_time = Instant::now();
    let total = emails.len() as i32;
    info!("Received bulk email request for {} recipients", total);
    
    let mut successes = Vec::new();
    let mut failures = Vec::new();
    let mut sent_count = 0i32;

    // Get campaign ID from first email and parse it to i32
    let campaign_id = emails.first()
        .map(|e| e.campaign_id.parse::<i32>())
        .transpose()
        .map_err(|_| ApiError::BadRequest("Invalid campaign_id format".into()))?;

    let campaign_repository = CampaignRepository::new(pool.get_ref().clone());
    let campaign_service = CampaignService::new(campaign_repository);

    if let Some(campaign_id) = campaign_id {
        // Set initial campaign status
        let update_dto = UpdateCampaignDto {
            id: Some(campaign_id),
            status: Some(CampaignStatus::Running),
            started_at: Some(Utc::now()),
            sent: Some(0),
            to_send: Some(total),
            ..Default::default()
        };
        campaign_service.update_campaign(update_dto).await?;
    }

    for (index, email) in emails.iter().enumerate() {
        let request = EmailRequest {
            to: email.to.clone(),
            subject: email.subject.clone(),
            content: email.content.clone(),
            metadata: Default::default(),
        };

        info!("Sending email {}/{} to: {}", index + 1, total, request.to);
        match send_with_retry(&service, request, MAX_RETRIES).await {
            Ok(response) => {
                info!("Successfully sent email to: {}", email.to);
                successes.push(response);
                sent_count += 1;

                if let Some(campaign_id) = campaign_id {
                    let update_dto = UpdateCampaignDto {
                        id: Some(campaign_id),
                        sent: Some(sent_count),
                        updated_at: Some(Utc::now()),
                        ..Default::default()
                    };
                    campaign_service.update_campaign(update_dto).await?;
                }
            },
            Err(e) => {
                error!("Failed to send email to {} after {} retries: {}", email.to, MAX_RETRIES, e);
                failures.push(format!("{}: {}", email.to, e));

                if let Some(campaign_id) = campaign_id {
                    let update_dto = UpdateCampaignDto {
                        id: Some(campaign_id),
                        status: Some(CampaignStatus::Paused),
                        sent: Some(sent_count),
                        updated_at: Some(Utc::now()),
                        ..Default::default()
                    };
                    campaign_service.update_campaign(update_dto).await?;
                }
            }
        }

        // Rate limiting delay between emails
        if (index as i32) < total - 1 {
            sleep(Duration::from_millis(RATE_LIMIT_DELAY_MS)).await;
        }
    }

    let duration_secs = start_time.elapsed().as_secs_f64();

    if let Some(campaign_id) = campaign_id {
        let update_dto = UpdateCampaignDto {
            id: Some(campaign_id),
            status: Some(if sent_count == total { CampaignStatus::Finished } else { CampaignStatus::Paused }),
            sent: Some(sent_count),
            updated_at: Some(Utc::now()),
            ..Default::default()
        };
        campaign_service.update_campaign(update_dto).await?;
    }

    info!("Bulk email send completed. Success: {}/{}. Duration: {:.2}s", 
          sent_count, total, duration_secs);

    Ok(HttpResponse::Ok().json(BulkEmailResponse {
        successes,
        failures,
        total: total as usize,
        success_count: sent_count as usize,
        duration_secs,
    }))
}
