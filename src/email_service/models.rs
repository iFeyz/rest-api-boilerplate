use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailRequest {
    pub to: String,
    pub subject: String,
    pub body: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BulkEmailRequest {
    pub emails: Vec<EmailRequest>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailResponse {
    pub message: String,
    pub success: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListEmailRequest {
    pub list_ids: Vec<i32>,
    pub subject: String,
    pub body: String,
    pub campaign_id: i32,
    pub sequence_email_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BulkEmailStats {
    pub total_subscribers: i32,
    pub successful_sends: i32,
    pub failed_sends: i32,
    pub failures: Vec<(String, String)>, // (email, error message)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CampaignEmailRequest {
    pub campaign_id: i32,
    pub list_ids: Vec<i32>,
    pub template_id: Option<i32>,  // Optional if campaign already has template
    pub schedule_at: Option<DateTime<Utc>>, // Optional for immediate sending
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CampaignEmailStats {
    pub campaign_id: i32,
    pub total_subscribers: i32,
    pub successful_sends: i32,
    pub failed_sends: i32,
    pub failures: Vec<(String, String)>,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}