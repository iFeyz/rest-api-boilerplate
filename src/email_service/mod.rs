pub mod error;
pub mod service;
pub mod models;
pub mod config;

pub use self::service::EmailService;
pub use self::models::EmailRequest;
pub use self::error::EmailError;