pub mod subscriber;
pub mod lists;
pub mod template;
pub mod subscriber_list;
pub mod campaign;
pub mod campaign_list;
pub mod send_email;
pub mod sequence_email;
use actix_web::web;
pub mod email_views;
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(subscriber::config());
    cfg.service(lists::config());
    cfg.service(template::config());
    cfg.service(subscriber_list::config());
    cfg.service(campaign::config());
    cfg.service(campaign_list::config());
    cfg.service(sequence_email::config());
    cfg.service(email_views::config());
}


