pub mod subscriber;
pub mod lists;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(subscriber::config());
    cfg.service(lists::config());
}

