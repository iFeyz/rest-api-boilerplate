use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Template {
    pub id: i32,
    pub name: String,
    #[sqlx(rename = "type")]
    pub template_type: Option<TemplateType>,
    pub subject: String,
    pub body: String,
    pub is_default: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug , Serialize , Deserialize , sqlx::Type , Clone , Default)]
#[sqlx(type_name = "template_type", rename_all = "lowercase")]
pub enum TemplateType {
    #[default]
    Campaign,
    Tx,
}

impl ToString for TemplateType {
    fn to_string(&self) -> String {
        match self {
            TemplateType::Campaign => "campaign".to_string(),
            TemplateType::Tx => "tx".to_string(),
        }
    }
}

#[derive(Debug , Deserialize)]
pub struct PaginationDto {
    #[serde(default)]
    pub query: Option<String>,

    #[serde(rename = "order_by")]
    #[serde(default = "default_template_order_by")]
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

fn default_order() -> String {
    "created_at".to_string()
}

fn default_page() -> i32 {
    1
}

fn default_per_page() -> i32 {
    10
}

#[derive(Debug  , Deserialize)]
pub struct CreateTemplateDto {
    pub name: String,
    pub template_type: TemplateType,
    pub subject: String,
    pub body: String,
    pub is_default: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTemplateDto {
    pub name: Option<String>,
    pub subject: Option<String>,
    pub template_type: Option<TemplateType>,
    pub body: Option<String>,
    pub is_default: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetTemplateDto {
    pub id: i32,
}