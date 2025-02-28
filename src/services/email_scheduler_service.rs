use tokio::sync::Mutex;
use std::sync::Arc;
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::{info, error};
use std::time::Instant;
use lettre::SmtpTransport;
use chrono::Utc;

use crate::{
    services::{
        sequence_emails_service::SequenceEmailService,
        subscriber_service::SubscriberService,
    },
    email_service::{
        models::{EmailWorker, EmailRequest},
        service::EmailService,
    },
    error::ApiError,
};

pub struct EmailSchedulerService {
    is_running: Arc<Mutex<bool>>,
    sequence_email_service: Arc<SequenceEmailService>,
    subscriber_service: Arc<SubscriberService>,
    workers: Vec<EmailWorker<SmtpTransport>>,
}

impl EmailSchedulerService {
    pub fn new(
        sequence_email_service: Arc<SequenceEmailService>,
        subscriber_service: Arc<SubscriberService>,
        workers: Vec<EmailWorker<SmtpTransport>>,
    ) -> Self {
        Self {
            is_running: Arc::new(Mutex::new(false)),
            sequence_email_service,
            subscriber_service,
            workers,
        }
    }

    pub async fn start_scheduler(&self) -> Result<(), ApiError> {
        let is_running = self.is_running.clone();
        let workers = self.workers.clone();
        let sequence_email_service = self.sequence_email_service.clone();
        let subscriber_service = self.subscriber_service.clone();

        let sched = JobScheduler::new().await.map_err(|e| {
            ApiError::Internal(format!("Failed to create job scheduler: {}", e))
        })?;

        sched.add(Job::new_async("0 */5 * * * *", move |_uuid, _l| {
            let is_running = is_running.clone();
            let workers = workers.clone();
            let sequence_email_service = sequence_email_service.clone();
            let subscriber_service = subscriber_service.clone();

            Box::pin(async move {
                let mut running = is_running.lock().await;
                if *running {
                    info!("Previous email batch still running, skipping this run");
                    return;
                }
                *running = true;

                info!("Starting scheduled email batch");
                let start_time = Instant::now();

                // Get pending emails from sequence
                match sequence_email_service.get_pending_emails().await {
                    Ok(pending_emails) => {
                        let worker = &workers[0]; // Using first worker for simplicity
                        let email_requests: Vec<EmailRequest> = pending_emails
                            .into_iter()
                            .map(|email| EmailRequest {
                                to: email.recipient_email,
                                subject: email.subject,
                                content: email.content,
                            })
                            .collect();

                        if let Err(e) = worker.send_batch(email_requests).await {
                            error!("Error processing email batch: {:?}", e);
                        } else {
                            info!("Successfully processed email batch");
                        }
                    }
                    Err(e) => {
                        error!("Failed to get pending emails: {:?}", e);
                    }
                }

                info!("Email batch completed in {:?}", start_time.elapsed());
                *running = false;
            })
        }).map_err(|e| {
            ApiError::Internal(format!("Failed to create job: {}", e))
        })?).await.map_err(|e| {
            ApiError::Internal(format!("Failed to add job to scheduler: {}", e))
        })?;

        sched.start().await.map_err(|e| {
            ApiError::Internal(format!("Failed to start scheduler: {}", e))
        })?;

        Ok(())
    }
} 