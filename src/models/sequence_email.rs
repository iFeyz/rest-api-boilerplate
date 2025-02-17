use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use crate::models::campaign::ContentType;
use serde_json::Value as JsonValue;
use std::fmt;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct SequenceEmail {
    pub id: i32,
    pub campaign_id: i32,
    pub position: i32,
    pub subject: String,
    pub body: String,
    pub template_id: Option<i32>,
    pub content_type: ContentType,
    pub metadata: JsonValue,
    pub is_active: bool,
    pub send_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSequenceEmailDto {
    pub campaign_id: i32,
    pub position: i32,
    pub subject: String,
    pub body: String,
    pub template_id: Option<i32>,
    pub content_type: ContentType,
    pub metadata: JsonValue,
    pub is_active: bool,
    pub send_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateSequenceEmailDto {
    pub subject: Option<String>,
    pub body: Option<String>,
    pub template_id: Option<i32>,
    pub content_type: Option<ContentType>,
    pub metadata: Option<JsonValue>,
    pub is_active: Option<bool>,
    pub send_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginationDto {
    #[serde(default = "default_campaign_id")]
    pub campaign_id: i32,
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_per_page")]
    pub per_page: i64,
}

fn default_campaign_id() -> i32 {
    1
}

fn default_page() -> i64 {
    1
}

fn default_per_page() -> i64 {
    10
}

impl Default for PaginationDto {
    fn default() -> Self {
        Self {
            campaign_id: default_campaign_id(),
            page: default_page(),
            per_page: default_per_page(),
        }
    }
}

impl fmt::Display for PaginationDto {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "campaign_id: {}, page: {}, per_page: {}", 
            self.campaign_id, self.page, self.per_page)
    }
} 