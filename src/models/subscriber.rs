use serde::{Deserialize, Serialize};
use sqlx::types::JsonValue;
use sqlx::{FromRow, Type};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::fmt;

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "subscriber_status", rename_all = "lowercase")]
pub enum SubscriberStatus {
    Enabled,
    Disabled,
    Blocklisted,
}

impl fmt::Display for SubscriberStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status_str = match self {
            SubscriberStatus::Enabled => "enabled",
            SubscriberStatus::Disabled => "disabled",
            SubscriberStatus::Blocklisted => "blocklisted",
        };
        write!(f, "{}", status_str)
    }
}



#[derive(Debug, Serialize, Deserialize )]
// Make pagination params optional page & per_page
pub struct PaginationParams {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}
// Make pagination params optional page & per_page
impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            page: Some(1),
            per_page: Some(10),
            sort_by: Some("id".to_string()),
            sort_order: Some("ASC".to_string()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize , Default)]
pub struct SubscriberFilter {
    pub id: Option<i32>,
    pub uuid: Option<Uuid>,
    pub email: Option<String>,
    pub name: Option<String>,
    pub attribs: Option<JsonValue>,
    pub status: Option<SubscriberStatus>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug , Serialize , Deserialize)]
pub struct SubscriberParams {
    pub pagination: PaginationParams,
    pub filter: SubscriberFilter,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriberResponse<T> {
    pub items: Vec<T>,
    pub page: i64,
    pub per_page: i64,
}

#[derive(Debug, Deserialize)]
pub struct PaginationDto {
    #[serde(default)]
    pub query: Option<String>,
    
    #[serde(default)]
    pub list_id: Option<Vec<i32>>,
    
    #[serde(default)]
    pub subscriber_status: Option<SubscriberStatus>,
    
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

fn default_order_by() -> String {
    "created_at".to_string()
}

fn default_order() -> String {
    "DESC".to_string()
}

fn default_page() -> i32 {
    1
}

fn default_per_page() -> i32 {
    10
}


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Subscriber {
    pub id: i32,
    pub uuid: Uuid,
    pub email: String,
    pub name: Option<String>,
    pub attribs: JsonValue,
    pub status: SubscriberStatus,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateSubscriberDto {
    pub email: String,
    pub name: Option<String>,
    pub attribs: Option<JsonValue>,
}

#[derive(Debug, Deserialize)]
pub struct GetSubscriberDto {
    pub id: Option<i32>,
    pub email: Option<String>,
}

