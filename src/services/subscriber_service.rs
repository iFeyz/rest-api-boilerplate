use uuid::Uuid;
use serde_json::Value;

use crate::{
    error::ApiError,
    models::subscriber::{Subscriber, CreateSubscriberDto, PaginationDto},
    repositories::subscriber_repository::SubscriberRepository
};

pub struct SubscriberService {
    repository: SubscriberRepository
}

impl SubscriberService {
    pub fn new(repository: SubscriberRepository) -> Self {
        Self { repository }
    }

    pub async fn create_subscriber(&self, dto: CreateSubscriberDto) -> Result<Subscriber, ApiError> {
        println!("Creating subscriber: {:?}", dto);
        self.repository.create(dto).await
    }

    pub async fn get_subscriber(&self, id_or_email: String) -> Result<Option<Subscriber>, ApiError> {
        // Try to parse as integer for ID lookup
        if let Ok(id) = id_or_email.parse::<i32>() {
            println!("Getting subscriber by ID: {:?}", id);
            self.repository.find_by_id(id).await
        } else {
            // If not an ID, treat as email
            println!("Getting subscriber by email: {:?}", id_or_email);
            self.repository.find_by_email(&id_or_email).await
        }
    }
    pub async fn delete_subscriber(&self, id_or_email: String) -> Result<Option<Subscriber>, ApiError> {
        if let Ok(id) = id_or_email.parse::<i32>() {
            println!("Deleting subscriber by ID: {:?}", id);
            self.repository.delete_by_id(id).await
        } else {
            println!("Deleting subscriber by email: {:?}", id_or_email);
            self.repository.delete_by_email(&id_or_email).await
        }
    }
    pub async fn get_subscribers(&self, params: PaginationDto) -> Result<Option<Vec<Subscriber>>, ApiError> {
        println!("Getting subscribers with params: {:?}", params);
        self.repository.find_all(&params).await
    }

    pub async fn update_subscriber(&self, id_or_email: String, subscriber: Subscriber) -> Result<Option<Subscriber>, ApiError> {
        if let Ok(id) = id_or_email.parse::<i32>() {
            println!("Updating subscriber by ID: {:?}", id);
            self.repository.update_by_id(id, subscriber).await
        } else {
            println!("Updating subscriber by email: {:?}", id_or_email);
            self.repository.update_by_email(&id_or_email, subscriber).await
        }
    }
}
