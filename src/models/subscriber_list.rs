use serde::{Deserialize, Serialize};
use sqlx::types::JsonValue;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "subscription_status", rename_all = "lowercase")]
pub enum SubscriptionStatus {
    Unconfirmed,
    Confirmed,
    Unsubscribed,
}

impl ToString for SubscriptionStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Unconfirmed => "unconfirmed".to_string(),
            Self::Confirmed => "confirmed".to_string(),
            Self::Unsubscribed => "unsubscribed".to_string(),
        }
    }
}

impl Default for SubscriptionStatus {
    fn default() -> Self {
        Self::Unconfirmed
    }
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct SubscriberList {
    pub subscriber_id: i32,
    pub list_id: Option<i32>,
    pub meta: JsonValue,
    pub status: SubscriptionStatus,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateSubscriberListDto {
    pub subscriber_id: i32,
    pub list_id: i32,
    pub meta: JsonValue,
    pub status: SubscriptionStatus,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateSubscriberListDto {
    pub meta: Option<JsonValue>,
    pub status: Option<SubscriptionStatus>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetSubscriberListDto {
    pub subscriber_id: i32,
    pub list_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct PaginationDto {
    #[serde(default)]
    pub subscriber_id: Option<i32>,
    
    #[serde(default)]
    pub list_id: Option<i32>,
    
    #[serde(default)]
    pub status: Option<SubscriptionStatus>,
    
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



