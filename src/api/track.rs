use actix_web::{web, HttpResponse, get};
use crate::error::ApiError;
use crate::models::subscriber::{Subscriber , UpdateSubscriberDto};
use crate::services::subscriber_service::SubscriberService;

pub fn config() -> actix_web::Scope {
    web::scope("/api/track")
        .service(track_open)
}

#[get("/{campaign_id}/{subscriber_id}")]
pub async fn track_open(
    service: web::Data<SubscriberService>,
    path: web::Path<(String, String)>,
) -> Result<HttpResponse, ApiError> {
    let (campaign_id, subscriber_id) = path.into_inner();
    let subscriber = service.update_subscriber(UpdateSubscriberDto {
        id: Some(subscriber_id.parse::<i32>().unwrap()),
        attribs: Some(json!({
            "ip": 
            "user_agent": 
        })),
        ..Default::default()
    }).await?;
    Ok(HttpResponse::Ok().body("Tracking pixel"))
}