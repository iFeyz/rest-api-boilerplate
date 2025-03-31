use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct SubscriberSequenceProgress {
    pub id: i32,
    pub subscriber_id: i32,
    pub campaign_id: i32,
    pub list_id: i32,
    pub joined_at: DateTime<Utc>,
    pub current_position: i32,
    pub last_email_sent_at: Option<DateTime<Utc>>,
    pub next_email_scheduled_at: Option<DateTime<Utc>>,
    pub completed: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSequenceProgressDto {
    pub subscriber_id: i32,
    pub campaign_id: i32,
    pub list_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateSequenceProgressDto {
    pub current_position: Option<i32>,
    pub last_email_sent_at: Option<DateTime<Utc>>,
    pub next_email_scheduled_at: Option<DateTime<Utc>>,
    pub completed: Option<bool>,
}

// Extension du modèle sequence_email existant
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DelayType {
    #[serde(rename = "absolute")]
    Absolute,
    #[serde(rename = "after_join")]
    AfterJoin,
    #[serde(rename = "after_previous")]
    AfterPrevious,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DelayUnit {
    #[serde(rename = "minutes")]
    Minutes,
    #[serde(rename = "hours")]
    Hours,
    #[serde(rename = "days")]
    Days,
}