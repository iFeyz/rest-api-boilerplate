use serde::{Deserialize, Serialize};
#[derive(Debug , Clone , Serialize , Deserialize)]
#[serde(tag = "type")]
pub enum EmailProviderConfig {
    Smtp(SmtpConfig),
    AwsSes(AwsSesConfig),
}


#[derive(Debug , Clone , Serialize , Deserialize)]
pub struct SmtpConfig {
    pub server: String,
    pub port: i32,
    pub username: String,
    pub password: String,
    pub sender_email: String,
}

#[derive(Debug , Clone , Serialize , Deserialize)]
pub struct AwsSesConfig {
    pub region: String,
    pub sender_email: String,
}

impl Default for SmtpConfig {
    fn default() -> Self {
        Self {
            server: "ssl0.ovh.net".to_string(),
            port: 465,
            username: "info@wayfe.net".to_string(),
            password: "JFHSUifhu8fida99".to_string(),
            sender_email: "info@wayfe.net".to_string(),
        }
    }
} 

