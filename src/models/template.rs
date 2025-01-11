use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Template {
    pub id: i32,
    pub name: String,
    pub subject: String,
    pub body: String,
    pub is_default: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug , Deserialize)]
pub struct PaginationDto {
    #[serde(default)]
    pub query: Option<String>,

    #[serde(default)]
    pub type: Option<TemplateType>,

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

fn default_template_order_by() -> String {
    "created_at".to_string()
}

fn default_template_order() -> String {
    "DESC".to_string()
}

fn default_template_page() -> i32 {
    1
}

fn default_template_per_page() -> i32 {
    10
}

#[derive(Debug  , Deserialize)]
pub struct CreateTemplateDto {
    pub name: String,
    pub subject: String,
    pub body: String,
    pub is_default: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTemplateDto {
    pub name: Option<String>,
    pub subject: Option<String>,
    pub body: Option<String>,
    pub is_default: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetTemplateDto {
    pub id: i32,
}