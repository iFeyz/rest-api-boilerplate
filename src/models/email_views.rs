use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime , Utc};
use serde_json::Value as JsonValue;

#[derive(Debug , Serialize , Deserialize , FromRow)]
pub struct EmailView {
    pub id: i32,
    pub sequence_email_id: Option<i32>,
    pub subscriber_id : Option<i32>,
    pub opened_at: Option<DateTime<Utc>>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub country: Option<String>,
    pub city: Option<String>,
    pub region: Option<String>,
    pub latitude: Option<String>,
    pub longitude: Option<String>,
    pub metadata: JsonValue,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug , Serialize , Deserialize , Default)]
pub struct CreateEmailViewDto {
    pub sequence_email_id: i32,
    pub subscriber_id: i32,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub country: Option<String>,
    pub city: Option<String>,
    pub region: Option<String>,
    pub latitude: Option<String>,
    pub longitude: Option<String>,
    pub metadata: Option<JsonValue>,
}

#[derive(Debug , Serialize , Deserialize)]
pub struct GetEmailViewDto {
    pub id: Option<i32>,
    pub sequence_email_id: Option<i32>,
    pub subscriber_id: Option<i32>,
}


#[derive(Debug , Serialize , Deserialize)]
pub struct PaginationDto {
    #[serde(default)]
    pub query: Option<String>,
    #[serde(default)]
    pub sequence_email_id: Option<i32>,
    #[serde(default)]
    pub subscriber_id: Option<i32>,
    #[serde(default)]
    pub opened_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub ip_address: Option<String>,
    #[serde(default)]
    pub user_agent: Option<String>,
    #[serde(default)]
    pub country: Option<String>,
    #[serde(default)]
    pub city: Option<String>,
    #[serde(default)]
    pub region: Option<String>,

        
    #[serde(rename = "order_by")]
    #[serde(default = "default_order_by")]
    pub order_by: String,
    
    #[serde(default = "default_order")]
    pub order: String,
    
    #[serde(default = "default_page")]
    pub page: i32,
    
    #[serde(rename = "per_page")]
    #[serde(default = "default_per_page")]
    pub per_page: i32,
}

fn default_order_by() -> String { "created_at".to_string() }
fn default_order() -> String { "DESC".to_string() }
fn default_page() -> i32 { 1 }
fn default_per_page() -> i32 { 10 }
