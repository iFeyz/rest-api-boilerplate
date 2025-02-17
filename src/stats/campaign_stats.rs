use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct CampaignStats {
    // Campaign Info
    pub campaign_id: i32,
    pub campaign_name: String,
    pub status: String,
    pub start_date: Option<DateTime<Utc>>,
    
    // Overall Stats
    pub total_subscribers: i64,
    pub total_sent: i64,
    pub total_opens: i64,
    pub unique_opens: i64,
    pub open_rate: f64,
    
    // Sequence Stats
    pub total_sequence_emails: i64,
    pub sequence_stats: Vec<SequenceEmailStats>,
    
    // Geographic Stats
    pub country_stats: Vec<CountryStats>,
    pub city_stats: Vec<CityStats>,
    
    // Time-based Stats
    pub opens_by_hour: Vec<TimeStats>,
    pub opens_by_day: Vec<TimeStats>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SequenceEmailStats {
    pub sequence_email_id: i32,
    pub position: i32,
    pub subject: String,
    pub status: String,
    pub sent_at: Option<DateTime<Utc>>,
    pub total_opens: i64,
    pub unique_opens: i64,
    pub open_rate: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CountryStats {
    pub country: String,
    pub total_opens: i64,
    pub unique_opens: i64,
    pub percentage: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CityStats {
    pub city: String,
    pub country: String,
    pub total_opens: i64,
    pub unique_opens: i64,
    pub percentage: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeStats {
    pub time_period: String,
    pub total_opens: i64,
    pub percentage: f64,
}

