use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: Uuid,
    pub email : String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at : DateTime<Utc>,
}

#[derive(Debug , Deserialize)]
pub struct CreateUserDto {
    pub email : String,
    pub name : String,
}

#[derive(Debug , Deserialize)]
pub struct UpdateUserDto {
    pub email : Option<String>,
    pub name : Option<String>,
}

#[derive(Debug , Deserialize)]
pub struct GetUserDto {
    pub id : Uuid,
}