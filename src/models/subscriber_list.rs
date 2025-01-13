use serde::{Deserialize, Serialize};
use sqlx::types::JsonValue;
use sqlx::{FromRow, Type};
use chrono::{DateTime, Utc};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "subscription_status", rename_all = "lowercase")]
pub enum SubscriptionStatus {
    Unconfirmed,
    Confirmed,
    Unsubscribed,
}   


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


#[derive(Debug ,  Serialize , Deserialize , FromRow)]
pub struct SubscriberList {
    pub subscriber_id: i32,
    pub list_id: Option<i32>,
    pub meta: JsonValue,
    pub status: SubscriptionStatus,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug , Deserialize)]
pub struct CreateSubscriberListDto {
    pub subscriber_id: i32,
    pub list_id: i32,
    pub meta: JsonValue,
    pub status: SubscriptionStatus,
}

#[derive(Debug , Deserialize , Serialize)]
pub struct UpdateSubscriberListDto {
    pub meta: Option<JsonValue>,
    pub status: Option<SubscriptionStatus>,
}

#[derive(Debug , Deserialize , Serialize)]
pub struct GetSubscriberListDto {
    pub subscriber_id: i32,
    pub list_id: i32,
}



