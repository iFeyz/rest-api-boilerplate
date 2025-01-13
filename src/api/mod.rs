pub mod subscriber;
pub mod lists;
pub mod template;
pub mod subscriber_list;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(subscriber::config());
    cfg.service(lists::config());
    cfg.service(template::config());
    cfg.service(subscriber_list::config());
}

