use thiserror::Error;
use lettre::transport::smtp::Error as SmtpError;

#[derive(Debug, Error)]
pub enum EmailError {
    #[error("SMTP error: {0}")]
    SmtpError(#[from] SmtpError),

    #[error("Invalid email address: {0}")]
    InvalidAddress(String),

    #[error("Template error: {0}")]
    TemplateError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Execution error: {0}")]
    ExecutionError(String),
}