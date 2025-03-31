use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use crate::models::campaign::ContentType;
use serde_json::Value as JsonValue;
use std::fmt;

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "sequence_email_status", rename_all = "lowercase")]
pub enum SequenceEmailStatus {
    Draft,
    Sending,
    Sent,
    Failed,
}

impl Default for SequenceEmailStatus {
    fn default() -> Self {
        Self::Draft
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DelayType {
    #[serde(rename = "absolute")]
    Absolute,
    #[serde(rename = "after_join")]
    AfterJoin,
    #[serde(rename = "after_previous")]
    AfterPrevious,
}

impl Default for DelayType {
    fn default() -> Self {
        Self::Absolute
    }
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

impl Default for DelayUnit {
    fn default() -> Self {
        Self::Minutes
    }
}

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
    pub status: SequenceEmailStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub delay_type: String,
    pub delay_value: Option<i32>,
    pub delay_unit: Option<String>,
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
    #[serde(default)]
    pub status: SequenceEmailStatus,
    #[serde(default)]
    pub delay_type: String,
    pub delay_value: Option<i32>,
    pub delay_unit: Option<String>,
}

impl Default for CreateSequenceEmailDto {
    fn default() -> Self {
        Self {
            status: SequenceEmailStatus::Draft,
            campaign_id: 1,
            position: 1,
            subject: String::new(),
            body: String::new(),
            template_id: None,
            content_type: ContentType::Html,
            metadata: JsonValue::Null,
            is_active: true,
            send_at: None,
            delay_type: "absolute".to_string(),
            delay_value: None,
            delay_unit: None,
        }
    }
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
    pub delay_type: Option<String>,
    pub delay_value: Option<i32>,
    pub delay_unit: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginationDto {
    #[serde(default = "default_campaign_id")]
    pub campaign_id: i32,
    #[serde(default = "default_page")]
    pub page: Option<i64>,
    #[serde(default = "default_per_page")]
    pub limit: Option<i64>,
}

fn default_campaign_id() -> i32 {
    1
}

fn default_page() -> Option<i64> {
    Some(1)
}

fn default_per_page() -> Option<i64> {
    Some(10)
}

impl Default for PaginationDto {
    fn default() -> Self {
        Self {
            campaign_id: default_campaign_id(),
            page: default_page(),
            limit: default_per_page(),
        }
    }
}

impl fmt::Display for PaginationDto {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "campaign_id: {}, page: {:?}, limit: {:?}", 
            self.campaign_id, self.page, self.limit)
    }
} 