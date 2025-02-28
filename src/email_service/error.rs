use thiserror::Error;
use lettre::transport::smtp::Error as SmtpError;

#[derive(Debug, Error)]
pub enum EmailError {
    #[error("SMTP error: {0}")]
    SmtpError(SmtpError),
    #[error("Execution error: {0}")]
    ExecutionError(String),
    #[error("Message error: {0}")]
    MessageError(String),
}