use actix_web::{HttpResponse , ResponseError};
use thiserror::Error;
use crate::email_service::error::EmailError;

#[derive(Debug , Error)]
pub enum ApiError {
    #[error("Database error : {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Not found")]
    NotFound,
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Email error: {0}")]
    EmailError(#[from] EmailError),
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::DatabaseError(_) => {
                HttpResponse::InternalServerError().json("Internal server error")
            }
            ApiError::NotFound => {
                HttpResponse::NotFound().json("Not found")
            }
            ApiError::BadRequest(msg) => {
                HttpResponse::BadRequest().json(msg)
            }
            ApiError::EmailError(e) => {
                HttpResponse::InternalServerError().json(e.to_string())
            }
        }
    }
}