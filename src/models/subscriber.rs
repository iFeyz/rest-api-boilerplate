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

impl ToString for SubscriberStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Enabled => "enabled".to_string(),
            Self::Disabled => "disabled".to_string(),
            Self::Blocklisted => "blocklisted".to_string(),
        }
    }
}
impl Default for SubscriberStatus {
    fn default() -> Self {
        Self::Enabled
    }
    
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

