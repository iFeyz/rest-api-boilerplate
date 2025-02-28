use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use chrono::{DateTime, Utc};
use sqlx::FromRow;
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
    pub updated_at: DateTime<Utc>
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
    pub send_at: Option<DateTime<Utc>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateSequenceEmailDto {
    pub subject: Option<String>,
    pub body: Option<String>,
    pub template_id: Option<i32>,
    pub content_type: Option<ContentType>,
    pub metadata: Option<JsonValue>,
    pub is_active: Option<bool>,
    pub send_at: Option<DateTime<Utc>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteSequenceEmailDto {
    pub id: i32
}

#[derive(Debug, Serialize, Deserialize , Clone , Default)]
pub struct PaginationDto {
    #[serde(default)]
    pub query: Option<String>,

    #[serde(default)]
    pub campaign_id: Option<i32>,

    #[serde(default = "default_page")]
    pub page: i32,
    
    #[serde(rename = "per_page")]
    #[serde(default = "default_per_page")]
    pub per_page: i32,
    
    #[serde(rename = "order_by")]
    #[serde(default = "default_order_by")]
    pub order_by: String,
    
    #[serde(default = "default_order")]
    pub order: String,
}

impl PaginationDto {
    pub fn to_string(&self) -> String {
        format!(" campaign_id: {}, page: {}, per_page: {}, order_by: {}, order: {}", self.campaign_id.unwrap_or(0), self.page, self.per_page, self.order_by, self.order)
    }
}

fn default_page() -> i32 { 1 }
fn default_per_page() -> i32 { 10 }
fn default_order_by() -> String { "created_at".to_string() }
fn default_order() -> String { "DESC".to_string() }

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "content_type", rename_all = "snake_case")]
pub enum ContentType {
    Richtext,
    Html,
    Plain,
    Markdown
}

