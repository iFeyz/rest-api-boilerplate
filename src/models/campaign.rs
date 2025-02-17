use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use sqlx::types::JsonValue;
use std::fmt;
use sqlx::Type;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Campaign {
    pub id: i32,
    pub uuid: Uuid,
    pub name: String,
    pub subject: String,
    pub from_email: String,
    pub status: CampaignStatus,
    pub campaign_type: CampaignType,
    pub tags: Option<Vec<String>>,
    pub messenger: String,
    pub headers: JsonValue,
    pub to_send: i32,
    pub sent: i32,
    pub max_subscriber_id: i32,
    pub last_subscriber_id: i32,
    pub archive: bool,
    pub archive_slug: Option<String>,
    pub archive_template_id: Option<i32>,
    pub archive_meta: JsonValue,
    pub started_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub sequence_start_date: Option<DateTime<Utc>>,
    pub sequence_end_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "campaign_status", rename_all = "lowercase")]
pub enum CampaignStatus {
    Draft,
    Running,
    Scheduled,
    Paused,
    Cancelled,
    Finished,
}

impl ToString for CampaignStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Draft => "draft".to_string(),
            Self::Running => "running".to_string(),
            Self::Scheduled => "scheduled".to_string(),
            Self::Paused => "paused".to_string(),
            Self::Cancelled => "cancelled".to_string(),
            Self::Finished => "finished".to_string(),
        }
    }
}

impl Default for CampaignStatus {
    fn default() -> Self {
        Self::Draft
    }
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "campaign_type", rename_all = "lowercase")]
pub enum CampaignType {
    Regular,
    Optin,
}

impl Default for CampaignType {
    fn default() -> Self {
        Self::Regular
    }
}

#[derive(Debug, Serialize, Deserialize, Type, Clone, Copy, PartialEq)]
#[sqlx(type_name = "content_type", rename_all = "lowercase")]
pub enum ContentType {
    Richtext,
    Html,
    Plain,
    Markdown,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCampaignDto {
    pub name: String,
    pub subject: String,
    pub from_email: String,
    pub campaign_type: CampaignType,
    #[serde(default)]
    pub status: CampaignStatus,
    #[serde(default)]
    pub tags: Option<Vec<String>>,
    #[serde(default = "default_messenger")]
    pub messenger: String,
    #[serde(default = "default_headers")]
    pub headers: JsonValue,
}

fn default_messenger() -> String {
    "smtp".to_string()
}

fn default_headers() -> JsonValue {
    serde_json::json!({})
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct CampaignDto {
    pub id: Option<i32>,
    pub uuid: Option<Uuid>,
    pub name: Option<String>,
    pub subject: Option<String>,
    pub from_email: Option<String>,
    pub status: Option<CampaignStatus>,
    pub campaign_type: Option<CampaignType>,
    pub tags: Option<Vec<String>>,
    pub messenger: Option<String>,
    pub headers: Option<JsonValue>,
    pub to_send: Option<i32>,
    pub sent: Option<i32>,
    pub max_subscriber_id: Option<i32>,
    pub last_subscriber_id: Option<i32>,
    pub archive: Option<bool>,
    pub archive_slug: Option<String>,
    pub archive_template_id: Option<i32>,
    pub archive_meta: Option<JsonValue>,
    pub started_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct DeleteCampaignDto {
    pub id: Option<i32>,
    pub uuid: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginationParams {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

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
pub struct CampaignFilter {
    pub id: Option<i32>,
    pub uuid: Option<Uuid>,
    pub name: Option<String>,
    pub subject: Option<String>,
    pub from_email: Option<String>,
    pub status: Option<CampaignStatus>,
    pub campaign_type: Option<CampaignType>,
    pub tags: Option<String>,
    pub messenger: Option<String>,
    pub headers: Option<JsonValue>,
    pub to_send: Option<i32>,
    pub sent: Option<i32>,
    pub max_subscriber_id: Option<i32>,
    pub last_subscriber_id: Option<i32>,
    pub archive: Option<bool>,
    pub archive_slug: Option<String>,
    pub archive_template_id: Option<i32>,
    pub archive_meta: Option<JsonValue>,
    pub started_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub sequence_start_date: Option<DateTime<Utc>>,
    pub sequence_end_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CampaignParams {
    pub pagination: PaginationParams,
    pub filter: CampaignFilter,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UpdateCampaignDto {
    #[serde(skip)]
    pub id: Option<i32>,
    pub uuid: Option<Uuid>,
    pub name: Option<String>,
    pub subject: Option<String>,
    pub from_email: Option<String>,
    pub status: Option<CampaignStatus>,
    pub campaign_type: Option<CampaignType>,
    pub tags: Option<Vec<String>>,
    pub messenger: Option<String>,
    pub headers: Option<JsonValue>,
    pub to_send: Option<i32>,
    pub sent: Option<i32>,
    pub max_subscriber_id: Option<i32>,
    pub last_subscriber_id: Option<i32>,
    pub archive: Option<bool>,
    pub archive_slug: Option<String>,
    pub archive_template_id: Option<i32>,
    pub archive_meta: Option<JsonValue>,
    pub started_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub sequence_start_date: Option<DateTime<Utc>>,
    pub sequence_end_date: Option<DateTime<Utc>>,
}   

#[derive(Debug, Serialize, Deserialize)]
pub struct CampaignResponse<T> {
    pub items: Vec<T>,
    pub page: i64,
    pub per_page: i64,
}