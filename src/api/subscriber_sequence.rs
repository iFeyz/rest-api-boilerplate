use actix_web::{web, HttpResponse, HttpRequest};
use crate::error::ApiError;
use crate::services::sequence_optin_service::SequenceOptinService;
use crate::services::subscriber_service::SubscriberService;
use std::sync::Arc;
use prometheus::IntCounterVec;
use crate::monitoring::Metrics;

pub fn config(metrics: Arc<Metrics>) -> actix_web::Scope {
    let counter = IntCounterVec::new(
        prometheus::opts!("api_subscriber_sequence_requests_total", "Total number of requests to subscriber sequence endpoints"),
        &["endpoint"]
    ).unwrap();
    
    // Register the counter with the metrics registry
    if let Err(e) = metrics.registry.register(Box::new(counter.clone())) {
        tracing::warn!("Failed to register subscriber sequence counter: {}", e);
    }
    
    let counter = Arc::new(counter);
    
    web::scope("/subscriber-sequence")
        .route("{subscriber_email}/lists/{list_id}", web::post().to({
            let counter = counter.clone();
            move |_req: HttpRequest, 
                  path: web::Path<(String, i32)>, 
                  sequence_service: web::Data<SequenceOptinService>,
                  subscriber_service: web::Data<SubscriberService>| {
                counter.with_label_values(&["add_subscriber_to_list_with_sequence"]).inc();
                async move {
                    let (subscriber_email, list_id) = path.into_inner();
                    
                    // First get or create the subscriber from the email
                    let subscriber = match subscriber_service.get_subscriber(subscriber_email.clone()).await {
                        Ok(Some(subscriber)) => subscriber,
                        Ok(None) => {
                            // Subscriber doesn't exist, create a new one
                            let create_dto = crate::models::subscriber::CreateSubscriberDto {
                                email: subscriber_email.clone(),
                                name: None,
                                attribs: None,
                            };
                            match subscriber_service.create_subscriber(create_dto).await {
                                Ok(subscriber) => subscriber,
                                Err(e) => return Err(e)
                            }
                        },
                        Err(e) => return Err(e)
                    };
                    
                    // Initialize the sequences opt-in
                    match sequence_service.initialize_sequences_for_subscriber(subscriber.id, list_id).await {
                        Ok(progress) => Ok::<HttpResponse, ApiError>(HttpResponse::Created().json(progress)),
                        Err(e) => Err(e)
                    }
                }
            }
        }))
}