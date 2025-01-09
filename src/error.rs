use actix_web::{HttpResponse , ResponseError};
use thiserror::Error;

#[derive(Debug , Error)]
pub enum ApiError {
    #[error("Database error : {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Not found")]
    NotFound,
    #[error("Bad request: {0}")]
    BadRequest(String),
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
        }
    }
}