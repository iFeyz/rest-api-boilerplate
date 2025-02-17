use async_trait::async_trait;
use crate::email_service::{models::EmailRequest, error::EmailError};
use crate::email_service::models::{BulkEmailStats, CampaignEmailStats};
use std::collections::HashSet;
use crate::error::ApiError;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::SmtpTransport;
use lettre::{Message, Transport};
use tokio::task;
use sqlx::PgPool;
use crate::models::subscriber::Subscriber;
use crate::models::subscriber::SubscriberStatus;
use serde_json;
use tracing;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ProviderFeature {
    Html,
    Attachments,
    Templates,
    Tracking,
    Scheduling,
    BulkSend,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ProviderType {
    Smtp,
    AwsSes,
    Mailgun,
    SendGrid,
    Mock,
}

#[async_trait]
pub trait EmailProvider: Send + Sync {
    async fn send(&self, email: EmailRequest) -> Result<String, EmailError>;
    fn provider_name(&self) -> &'static str;
    fn provider_type(&self) -> ProviderType;
    fn supported_features(&self) -> HashSet<ProviderFeature>;
    
    fn supports_feature(&self, feature: ProviderFeature) -> bool {
        self.supported_features().contains(&feature)
    }
}

#[derive(Clone)]
pub struct EmailService {
    smtp_client: SmtpTransport,
    from_email: String,
}

//
//** Create a new email service
//** Params : host , username , password , from_email
//** Return : EmailService
//
impl EmailService {
    pub fn new(host: String, username: String, password: String, from_email: String) -> Self {
        let creds = Credentials::new(username, password);
        
        let smtp_client = SmtpTransport::relay(&host)
            .unwrap()
            .credentials(creds)
            .build();

        Self {
            smtp_client,
            from_email,
        }
    }


    //
    //** Send an email
    //** Params : to , subject , body
    //** Return : Result<(), ApiError>
    //
    pub async fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<(), ApiError> {
        let email = Message::builder()
            .from(self.from_email.parse().unwrap())
            .to(to.parse().unwrap())
            .subject(subject)
            .header(ContentType::TEXT_HTML)
            .body(String::from(body))
            .unwrap();

        // Clone the necessary data for the spawned task
        let smtp_client = self.smtp_client.clone();
        
        // Spawn a blocking task for SMTP operations
        task::spawn_blocking(move || {
            smtp_client
                .send(&email)
                .map_err(|e| EmailError::SmtpError(e))
        })
        .await
        .map_err(|e| EmailError::ExecutionError(e.to_string()))?
        .map_err(ApiError::EmailError)?;

        Ok(())
    }

    // Optional: Method for sending multiple emails concurrently
    pub async fn send_bulk_emails(
        &self,
        emails: Vec<(String, String, String)>, // (to, subject, body)
    ) -> Vec<Result<(), ApiError>> {
        let futures: Vec<_> = emails
            .into_iter()
            .map(|(to, subject, body)| {
                // Clone self for the async closure
                let email_service = self.clone();
                async move {
                    email_service.send_email(&to, &subject, &body).await
                }
            })
            .collect();

        futures::future::join_all(futures).await
    }

    /// Send emails to multiple recipients in chunks of 10
    pub async fn send_batch_email(
        &self,
        recipients: Vec<String>,
        subject: &str,
        body: &str,
    ) -> Result<(), ApiError> {
        const CHUNK_SIZE: usize = 10;

        // Split recipients into chunks of 10
        for chunk in recipients.chunks(CHUNK_SIZE) {
            // Create a single email message with multiple recipients
            let mut email = Message::builder()
                .from(self.from_email.parse().unwrap())
                .subject(subject)
                .header(ContentType::TEXT_HTML);

            // Add all recipients in the current chunk
            for recipient in chunk {
                email = email.to(recipient.parse().unwrap());
            }

            // Add the body after adding all recipients
            let email = email.body(String::from(body))
                .map_err(|e| EmailError::MessageError(e.to_string()))?;

            // Clone the SMTP client for the spawned task
            let smtp_client = self.smtp_client.clone();

            // Spawn a blocking task for SMTP operations
            task::spawn_blocking(move || {
                smtp_client
                    .send(&email)
                    .map_err(EmailError::SmtpError)
            })
            .await
            .map_err(|e| EmailError::ExecutionError(e.to_string()))?
            .map_err(ApiError::EmailError)?;

            // Small delay between chunks to avoid overwhelming the SMTP server
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        }

        Ok(())
    }

    /// Creates a tracking pixel HTML for email tracking
    pub fn create_tracking_pixel(
        &self,
        campaign_id: i32,
        sequence_email_id: i32,
        subscriber_id: i32,
    ) -> String {
        let tracking_url = self.create_track_endpoint(campaign_id, sequence_email_id, subscriber_id);
        format!(
            r#"<img src="{}" width="1" height="1" alt="" style="display:none;border:0;width:1px;height:1px" />"#,
            tracking_url
        )
    }

    /// Creates a tracking endpoint URL
    pub fn create_track_endpoint(
        &self,
        campaign_id: i32,
        sequence_email_id: i32,
        subscriber_id: i32,
    ) -> String {
        format!(
            "{}/{}/{}/{}",
            "http://localhost:8080/api/email_views",
            subscriber_id,
            sequence_email_id,
            campaign_id
        )
    }

    /// Adds tracking to an HTML email body
    pub fn add_tracking_to_email(
        &self,
        body: &str,
        campaign_id: i32,
        sequence_email_id: i32,
        subscriber_id: i32,
    ) -> String {
        let tracking_pixel = self.create_tracking_pixel(campaign_id, sequence_email_id, subscriber_id);
        
        // If the email has a </body> tag, insert before it
        if body.contains("</body>") {
            body.replace("</body>", &format!("{}</body>", tracking_pixel))
        } else {
            // Otherwise append to the end
            format!("{}{}", body, tracking_pixel)
        }
    }

    /// Send an email with tracking
    pub async fn send_tracked_email(
        &self,
        to: &str,
        subject: &str,
        body: &str,
        campaign_id: i32,
        sequence_email_id: i32,
        subscriber_id: i32,
    ) -> Result<(), ApiError> {
        let tracked_body = self.add_tracking_to_email(
            body,
            campaign_id,
            sequence_email_id,
            subscriber_id,
        );

        self.send_email(to, subject, &tracked_body).await
    }

    // Update send_emails_to_lists to include tracking
    pub async fn send_emails_to_lists(
        &self,
        pool: &PgPool,
        list_ids: &[i32],
        subject: &str,
        body: &str,
        campaign_id: i32,
        sequence_email_id: i32,
    ) -> Result<BulkEmailStats, ApiError> {
        const FETCH_SIZE: i64 = 50;
        
        tracing::info!("Starting to send emails to lists: {:?}", list_ids);
        
        let mut stats = BulkEmailStats {
            total_subscribers: 0,
            successful_sends: 0,
            failed_sends: 0,
            failures: Vec::new(),
        };

        // Get total count first
        let total_count: i64 = sqlx::query_scalar!(
            r#"
            SELECT COUNT(DISTINCT s.id)::bigint
            FROM subscribers s
            JOIN subscriber_lists sl ON s.id = sl.subscriber_id
            WHERE sl.list_id = ANY($1)
            AND sl.status = 'confirmed'
            AND s.status = 'enabled'
            "#,
            list_ids
        )
        .fetch_one(pool)
        .await?
        .unwrap_or(0);

        tracing::info!("Found {} total subscribers to send to", total_count);
        stats.total_subscribers = total_count as i32;
        let mut offset: i64 = 0;

        loop {
            // Fetch next batch of subscribers
            let subscribers = sqlx::query_as!(
                Subscriber,
                r#"
                SELECT 
                    s.id, 
                    s.uuid, 
                    s.email, 
                    s.name, 
                    s.attribs,
                    s.status as "status: SubscriberStatus",
                    s.created_at,
                    s.updated_at
                FROM subscribers s
                JOIN subscriber_lists sl ON s.id = sl.subscriber_id
                WHERE sl.list_id = ANY($1)
                AND sl.status = 'confirmed'
                AND s.status = 'enabled'
                ORDER BY s.id
                LIMIT $2 OFFSET $3
                "#,
                list_ids,
                FETCH_SIZE,
                offset
            )
            .fetch_all(pool)
            .await?;

            if subscribers.is_empty() {
                break;
            }

            tracing::info!("Processing batch of {} subscribers", subscribers.len());

            // Create batch of emails with tracking pixels
            let mut batch_recipients = Vec::new();
            let mut tracked_body = String::new();

            for subscriber in &subscribers {
                // Add tracking pixel to email body
                tracked_body = self.add_tracking_to_email(
                    body,
                    campaign_id,
                    sequence_email_id,
                    subscriber.id,
                );
                batch_recipients.push(subscriber.email.clone());
            }

            // Send the batch
            match self.send_batch_email(batch_recipients, subject, &tracked_body).await {
                Ok(_) => {
                    stats.successful_sends += subscribers.len() as i32;
                    tracing::info!("Successfully sent batch of {} emails", subscribers.len());
                }   
                Err(e) => {
                    stats.failed_sends += subscribers.len() as i32;
                    let error_msg = format!("Failed to send batch: {}", e);
                    tracing::error!("{}", error_msg);
                    // Record the first email in batch for error tracking
                    if let Some(first_subscriber) = subscribers.first() {
                        stats.failures.push((first_subscriber.email.clone(), error_msg));
                    }
                }
            }

            offset += FETCH_SIZE;
        }

        tracing::info!(
            "Completed sending emails. Success: {}, Failed: {}", 
            stats.successful_sends, 
            stats.failed_sends
        );

        Ok(stats)
    }

    pub async fn send_campaign_emails(
        &self,
        pool: &PgPool,
        campaign_id: i32,
        list_ids: &[i32],
        template_id: Option<i32>,
    ) -> Result<CampaignEmailStats, ApiError> {
        // Get the email content (either from template or sequence email)
        let (subject, body) = if let Some(template_id) = template_id {
            let template = sqlx::query!(
                r#"SELECT subject, body FROM templates WHERE id = $1"#,
                template_id
            )
            .fetch_one(pool)
            .await?;
            (template.subject, template.body)
        } else {
            let sequence_email = sqlx::query!(
                r#"
                SELECT subject, body 
                FROM sequence_emails 
                WHERE campaign_id = $1 
                AND is_active = true
                AND (send_at IS NULL OR send_at <= NOW())
                ORDER BY position ASC
                LIMIT 1
                "#,
                campaign_id
            )
            .fetch_optional(pool)
            .await?
            .ok_or_else(|| ApiError::BadRequest("No active sequence email found for campaign".to_string()))?;
            
            (sequence_email.subject, sequence_email.body)
        };

        // Use send_emails_to_lists for the actual sending
        let bulk_stats = self.send_emails_to_lists(pool, list_ids, &subject, &body, campaign_id, 0).await?;

        // Convert BulkEmailStats to CampaignEmailStats
        let campaign_stats = CampaignEmailStats {
            campaign_id,
            total_subscribers: bulk_stats.total_subscribers,
            successful_sends: bulk_stats.successful_sends,
            failed_sends: bulk_stats.failed_sends,
            failures: bulk_stats.failures,
            started_at: Utc::now(),
            completed_at: Some(Utc::now()),
        };

        Ok(campaign_stats)
    }

    pub async fn schedule_campaign_emails(
        &self,
        pool: &PgPool,
        campaign_id: i32,
        list_ids: &[i32],
        template_id: Option<i32>,
        schedule_at: DateTime<Utc>,
    ) -> Result<(), ApiError> {
        // Update campaign status and schedule time
        sqlx::query!(
            r#"
            UPDATE campaigns
            SET status = 'scheduled',
                sequence_start_date = $1,
                archive_meta = jsonb_set(
                    COALESCE(archive_meta, '{}'::jsonb),
                    '{schedule}',
                    $2::jsonb
                )
            WHERE id = $3
            "#,
            schedule_at,
            serde_json::json!({
                "scheduled_at": schedule_at,
                "list_ids": list_ids,
                "template_id": template_id,
            }),
            campaign_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn process_scheduled_campaigns(&self, pool: &PgPool) -> Result<(), ApiError> {
        // Get campaigns that are scheduled and due to be sent
        let campaigns = sqlx::query!(
            r#"
            SELECT 
                c.id,
                c.archive_meta->>'schedule' as schedule
            FROM campaigns c
            JOIN sequence_emails se ON c.id = se.campaign_id
            WHERE c.status = 'scheduled' 
            AND c.sequence_start_date <= $1
            AND se.is_active = true
            GROUP BY c.id
            "#,
            Utc::now()
        )
        .fetch_all(pool)
        .await?;

        for campaign in campaigns {
            if let Some(schedule) = campaign.schedule {
                let schedule: serde_json::Value = serde_json::from_str(&schedule)?;
                let list_ids: Vec<i32> = serde_json::from_value(schedule["list_ids"].clone())?;
                let template_id: Option<i32> = serde_json::from_value(schedule["template_id"].clone())?;

                // Send the campaign
                match self.send_campaign_emails(pool, campaign.id, &list_ids, template_id).await {
                    Ok(_) => {
                        tracing::info!("Successfully processed scheduled campaign {}", campaign.id);
                    }
                    Err(e) => {
                        tracing::error!("Failed to process scheduled campaign {}: {}", campaign.id, e);
                        // Update campaign status to failed
                        sqlx::query!(
                            r#"
                            UPDATE campaigns
                            SET status = 'cancelled',
                                archive_meta = jsonb_set(
                                    COALESCE(archive_meta, '{}'::jsonb),
                                    '{error}',
                                    $1::jsonb
                                )
                            WHERE id = $2
                            "#,
                            serde_json::json!({
                                "error": e.to_string(),
                                "failed_at": Utc::now(),
                            }),
                            campaign.id
                        )
                        .execute(pool)
                        .await?;
                    }
                }
            }
        }

        Ok(())
    }
} 