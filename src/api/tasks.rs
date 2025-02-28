use actix_web::{web, HttpResponse, HttpRequest};
use prometheus::IntCounterVec;
use std::sync::Arc;
use crate::monitoring::Metrics;

pub fn config(metrics: Arc<Metrics>) -> actix_web::Scope {
    let counter = IntCounterVec::new(
        prometheus::opts!("api_tasks_requests_total", "Total number of requests to task endpoints"),
        &["endpoint"]
    ).unwrap();
    
    // Register the counter with the metrics registry, handling potential errors
    if let Err(e) = metrics.registry.register(Box::new(counter.clone())) {
        tracing::warn!("Failed to register tasks counter: {}", e);
        // Continue even if registration fails
    }
    
    let counter = Arc::new(counter);
    
    web::scope("/tasks")
        .route("", web::get().to({
            let counter = counter.clone();
            move |_req: HttpRequest| {
                counter.with_label_values(&["get_tasks"]).inc();
                async move {
                    // Placeholder for task functionality
                    HttpResponse::Ok().json(serde_json::json!({
                        "message": "Tasks endpoint placeholder"
                    }))
                }
            }
        }))
} 