use actix_web::{web, HttpResponse, HttpRequest};
use crate::email_service::{
    EmailService,
    models::{EmailRequest, BulkEmailRequest, EmailResponse, ListEmailRequest, BulkEmailStats, CampaignEmailRequest},
};
use sqlx::PgPool;
use chrono::Utc;
use prometheus::IntCounterVec;
use std::sync::Arc;
use crate::monitoring::Metrics;

pub fn config(cfg: &mut web::ServiceConfig, counter: Arc<IntCounterVec>) {
    cfg.service(
        web::scope("/email")
            .route("/send", web::post().to({
                let counter = counter.clone();
                move |_req: HttpRequest, email_service: web::Data<EmailService>, request: web::Json<EmailRequest>| {
                    counter.with_label_values(&["send_email"]).inc();
                    async move {
                        match email_service.send_email(&request.to, &request.subject, &request.body).await {
                            Ok(_) => HttpResponse::Ok().json(EmailResponse {
                                message: "Email sent successfully".to_string(),
                                success: true,
                            }),
                            Err(e) => HttpResponse::InternalServerError().json(EmailResponse {
                                message: format!("Failed to send email: {}", e),
                                success: false,
                            }),
                        }
                    }
                }
            }))
            .route("/send-bulk", web::post().to({
                let counter = counter.clone();
                move |_req: HttpRequest, email_service: web::Data<EmailService>, request: web::Json<BulkEmailRequest>| {
                    counter.with_label_values(&["send_bulk_emails"]).inc();
                    async move {
                        let emails = request.emails.iter().map(|email| {
                            (
                                email.to.clone(),
                                email.subject.clone(),
                                email.body.clone(),
                            )
                        }).collect();

                        let results = email_service.send_bulk_emails(emails).await;
                        
                        let failures: Vec<_> = results.iter()
                            .enumerate()
                            .filter_map(|(i, result)| {
                                result.as_ref().err().map(|e| (i, e.to_string()))
                            })
                            .collect();

                        if failures.is_empty() {
                            HttpResponse::Ok().json(EmailResponse {
                                message: "All emails sent successfully".to_string(),
                                success: true,
                            })
                        } else {
                            HttpResponse::BadRequest().json(EmailResponse {
                                message: format!("Some emails failed to send: {:?}", failures),
                                success: false,
                            })
                        }
                    }
                }
            }))
            .route("/send-to-lists", web::post().to({
                let counter = counter.clone();
                move |_req: HttpRequest, email_service: web::Data<EmailService>, pool: web::Data<PgPool>, request: web::Json<ListEmailRequest>| {
                    counter.with_label_values(&["send_to_lists"]).inc();
                    async move {
                        match email_service.send_emails_to_lists(
                            &pool,
                            &request.list_ids,
                            &request.subject,
                            &request.body,
                            request.campaign_id,
                            request.sequence_email_id,
                        ).await {
                            Ok(stats) => HttpResponse::Ok().json(stats),
                            Err(e) => HttpResponse::InternalServerError().json(EmailResponse {
                                message: format!("Failed to send emails: {}", e),
                                success: false,
                            }),
                        }
                    }
                }
            }))
            .route("/send-to-lists-campaign", web::post().to({
                let counter = counter.clone();
                move |_req: HttpRequest, email_service: web::Data<EmailService>, pool: web::Data<PgPool>, request: web::Json<CampaignEmailRequest>| {
                    counter.with_label_values(&["send_to_lists_campaign"]).inc();
                    async move {
                        if let Some(schedule_at) = request.schedule_at {
                            if schedule_at <= Utc::now() {
                                return HttpResponse::BadRequest().json(EmailResponse {
                                    message: "Schedule time must be in the future".to_string(),
                                    success: false,
                                });
                            }

                            // Schedule the campaign
                            match email_service.schedule_campaign_emails(
                                &pool,
                                request.campaign_id,
                                &request.list_ids,
                                request.template_id,
                                schedule_at,
                            ).await {
                                Ok(_) => HttpResponse::Ok().json(EmailResponse {
                                    message: format!("Campaign scheduled for {}", schedule_at),
                                    success: true,
                                }),
                                Err(e) => HttpResponse::InternalServerError().json(EmailResponse {
                                    message: format!("Failed to schedule campaign: {}", e),
                                    success: false,
                                }),
                            }
                        } else {
                            // Send immediately
                            match email_service.send_campaign_emails(
                                &pool,
                                request.campaign_id,
                                &request.list_ids,
                                request.template_id,
                            ).await {
                                Ok(stats) => HttpResponse::Ok().json(stats),
                                Err(e) => HttpResponse::InternalServerError().json(EmailResponse {
                                    message: format!("Failed to send campaign emails: {}", e),
                                    success: false,
                                }),
                            }
                        }
                    }
                }
            }))
    );
}

pub fn configure(metrics: Arc<Metrics>) -> impl FnOnce(&mut web::ServiceConfig) {
    let counter = IntCounterVec::new(
        prometheus::opts!("api_email_requests_total", "Total number of requests to email endpoints"),
        &["endpoint"]
    ).unwrap();
    
    // Register the counter with the metrics registry, handling potential errors
    if let Err(e) = metrics.registry.register(Box::new(counter.clone())) {
        tracing::warn!("Failed to register email counter: {}", e);
        // Continue even if registration fails
    }
    
    let counter = Arc::new(counter);
    
    move |cfg: &mut web::ServiceConfig| {
        config(cfg, counter);
    }
} 