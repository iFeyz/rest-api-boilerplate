pub mod subscriber;
pub mod lists;
pub mod template;
pub mod subscriber_list;
pub mod campaign;
pub mod campaign_list;
use actix_web::web;
pub mod email_views;
pub mod send_email;
pub mod sequence_email;
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(subscriber::config());
    cfg.service(lists::config());
    cfg.service(template::config());
    cfg.service(subscriber_list::config());
    cfg.service(campaign::config());
    cfg.service(campaign_list::config());
    cfg.service(email_views::config());
    cfg.configure(send_email::config);
}


