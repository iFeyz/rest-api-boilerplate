use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePostDto {
    pub title: String,
    pub content: String,
    pub user_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePostDto {
    pub title: Option<String>,
    pub content: Option<String>,
}