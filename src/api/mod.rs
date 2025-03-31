pub mod subscriber;
pub mod lists;
pub mod template;
pub mod subscriber_list;
pub mod campaign;
pub mod campaign_list;
pub mod email_views;
pub mod send_email;
pub mod campaign_stats;
pub mod sequence_email;
pub mod users;
pub mod auth;
pub mod tasks;
pub mod subscriber_sequence;
use actix_web::web;
use std::sync::Arc;
use crate::monitoring::Metrics;

pub fn config(cfg: &mut web::ServiceConfig, metrics: Arc<Metrics>) {
    // Configure all API routes with metrics
    cfg.service(lists::config(metrics.clone()));
    cfg.service(template::config(metrics.clone()));
    cfg.service(subscriber_list::config(metrics.clone()));
    cfg.service(campaign::config(metrics.clone()));
    cfg.service(campaign_list::config(metrics.clone()));
    cfg.service(sequence_email::config(metrics.clone()));
    cfg.service(email_views::config(metrics.clone()));
    cfg.service(campaign_stats::config(metrics.clone()));
    cfg.service(subscriber::config(metrics.clone()));
    cfg.service(users::config(metrics.clone()));
    cfg.service(auth::config(metrics.clone()));
    cfg.service(tasks::config(metrics.clone()));
    cfg.configure(send_email::configure(metrics.clone()));
    cfg.service(subscriber_sequence::config(metrics.clone()));
}


