use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use sqlx::types::JsonValue;
use std::fmt;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Campaign {
    pub id: i32,
    pub uuid: Uuid,
    pub name: String,
    pub subject: String,
    pub from_email: String,
    pub body: String,
    pub altbody: String,
    pub content_type: ContentType,
    pub send_at: Option<DateTime<Utc>>,
    pub status: CampaignStatus,
    pub campaign_type: CampaignType,
    pub tags: Option<Vec<String>>,
    pub messenger: String,
    pub template_id: Option<i32>,
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

impl ToString for CampaignType {
    fn to_string(&self) -> String {
        match self {
            Self::Regular => "regular".to_string(),
            Self::Optin => "optin".to_string(),
        }
    }
}

impl Default for CampaignType {
    fn default() -> Self {
        Self::Regular
    }
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "content_type", rename_all = "lowercase")]
pub enum ContentType {
    Richtext,
    Html,
    Plain,
    Markdown,
}

impl ToString for ContentType {
    fn to_string(&self) -> String {
        match self {
            Self::Richtext => "richtext".to_string(),
            Self::Html => "html".to_string(),
            Self::Plain => "plain".to_string(),
            Self::Markdown => "markdown".to_string(),
        }
    }
}
impl Default for ContentType {
    fn default() -> Self {
        Self::Richtext
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCampaignDto {
    pub name: String,
    pub subject: String,
    pub from_email: String,
    pub body: String,
    pub altbody: String,
    pub content_type: ContentType,
    pub send_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub status: CampaignStatus,
    #[serde(default)]
    pub campaign_type: CampaignType,
    #[serde(default)]
    pub tags: Option<Vec<String>>,
    #[serde(default = "default_messenger")]
    pub messenger: String,
    pub template_id: Option<i32>,
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
    pub body: Option<String>,
    pub altbody: Option<String>,
    pub content_type: Option<ContentType>,
    pub send_at: Option<DateTime<Utc>>,
    pub status: Option<CampaignStatus>,
    pub campaign_type: Option<CampaignType>,
    pub tags: Option<Vec<String>>,
    pub messenger: Option<String>,
    pub template_id: Option<i32>,
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
pub struct PaginationDto {
    #[serde(default)]
    pub query: Option<String>,
    #[serde(default)]
    pub id: Option<Vec<i32>>,
    #[serde(default)]
    pub status: Option<CampaignStatus>,
    #[serde(default)]
    pub campaign_type: Option<CampaignType>,
    #[serde(default)]
    pub tags: Option<Vec<String>>,
    #[serde(default)]
    pub messenger: Option<String>,

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

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCampaignDto {
    #[serde(skip)]
    pub id: Option<i32>,
    pub uuid: Option<Uuid>,
    pub name: Option<String>,
    pub subject: Option<String>,
    pub from_email: Option<String>,
    pub body: Option<String>,
    pub altbody: Option<String>,
    pub content_type: Option<ContentType>,
    pub send_at: Option<DateTime<Utc>>,
    pub status: Option<CampaignStatus>,
    pub campaign_type: Option<CampaignType>,
    pub tags: Option<Vec<String>>,
    pub messenger: Option<String>,
    pub template_id: Option<i32>,
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
