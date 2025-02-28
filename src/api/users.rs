use actix_web::{web, HttpResponse, HttpRequest};
use prometheus::IntCounterVec;
use std::sync::Arc;
use crate::monitoring::Metrics;

pub fn config(metrics: Arc<Metrics>) -> actix_web::Scope {
    let counter = IntCounterVec::new(
        prometheus::opts!("api_users_requests_total", "Total number of requests to user endpoints"),
        &["endpoint"]
    ).unwrap();
    
    // Register the counter with the metrics registry, handling potential errors
    if let Err(e) = metrics.registry.register(Box::new(counter.clone())) {
        tracing::warn!("Failed to register users counter: {}", e);
        // Continue even if registration fails
    }
    
    let counter = Arc::new(counter);
    
    web::scope("/users")
        .route("", web::get().to({
            let counter = counter.clone();
            move |_req: HttpRequest| {
                counter.with_label_values(&["get_users"]).inc();
                async move {
                    // Placeholder for user functionality
                    HttpResponse::Ok().json(serde_json::json!({
                        "message": "Users endpoint placeholder"
                    }))
                }
            }
        }))
} 