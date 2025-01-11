use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::types::JsonValue;
use uuid::Uuid;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone)]
#[sqlx(type_name = "list_type", rename_all = "lowercase")]
pub enum ListType {
    Public,
    Private,
    Temporary,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone)]
#[sqlx(type_name = "list_optin", rename_all = "lowercase")]
pub enum ListOptin {
    Single,
    Double,
}

impl Default for ListType {
    fn default() -> Self {
        Self::Public
    }
}

impl Default for ListOptin {
    fn default() -> Self {
        Self::Single
    }
}

impl ToString for ListType {
    fn to_string(&self) -> String {
        match self {
            Self::Public => "public".to_string(),
            Self::Private => "private".to_string(),
            Self::Temporary => "temporary".to_string()
        }
    }
}


impl ToString for ListOptin {
    fn to_string(&self)-> String {
        match self {
            Self::Single => "single".to_string(),
            Self::Double => "double".to_string()
        }
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct List {
    pub id: i32,
    pub uuid: Uuid,
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: ListType,
    pub optin: ListOptin,
    #[sqlx(default)]
    pub tags: Vec<String>,
    pub description: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateListDto {
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: ListType,
    pub optin: ListOptin,
    #[serde(default)]
    pub tags: Vec<String>,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct GetListDto {
    pub id: Option<i32>,
    pub name: Option<String>,
} 


#[derive(Debug, Deserialize)]
pub struct ListPaginationDto {
    #[serde(default)]
    pub query: Option<String>,

    #[serde(default)]
    #[serde(rename = "type")]
    pub r#type: Option<Vec<ListType>>,
    
    #[serde(default)]
    pub tags: Option<Vec<String>>,
    
    #[serde(rename = "order_by")]
    #[serde(default = "default_list_order_by")]
    pub order_by: String,
    
    #[serde(default = "default_list_order")]
    pub order: String,
    
    #[serde(default = "default_list_page")]
    pub page: i32,
    
    #[serde(rename = "per_page")]
    #[serde(default = "default_list_per_page")]
    pub per_page: i32,
}

fn default_list_order_by() -> String {
    "created_at".to_string()
}

fn default_list_order() -> String {
    "DESC".to_string()
}

fn default_list_page() -> i32 {
    1
}

fn default_list_per_page() -> i32 {
    10
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateListDto {
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Option<ListType>,
    pub optin: Option<ListOptin>,
    #[serde(default)]
    pub tags: Option<Vec<String>>,
    pub description: Option<String>,
}