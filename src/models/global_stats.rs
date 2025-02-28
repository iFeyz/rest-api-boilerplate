use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalStats {
    // Overall Numbers
    pub total_subscribers: i64,
    pub active_subscribers: i64,
    pub total_campaigns: i64,
    pub total_lists: i64,
    pub total_emails_sent: i64,
    pub total_opens: i64,
    pub global_open_rate: f64,

    // Campaign Stats
    pub campaigns_last_30_days: i64,
    pub active_campaigns: i64,
    pub completed_campaigns: i64,
    pub failed_campaigns: i64,

    // Subscriber Activity
    pub new_subscribers_today: i64,
    pub new_subscribers_this_week: i64,
    pub new_subscribers_this_month: i64,
    pub unsubscribes_this_month: i64,
    pub subscriber_growth_rate: f64,

    // Email Engagement
    pub opens_today: i64,
    pub opens_this_week: i64,
    pub opens_this_month: i64,
    pub average_opens_per_campaign: f64,
    
    // Time-based Stats
    pub peak_engagement_hour: i32,
    pub peak_engagement_day: String,
    
    // Geographic Stats
    pub top_countries: Vec<CountryEngagement>,
    pub top_cities: Vec<CityEngagement>,

    // List Stats
    pub average_list_size: f64,
    pub largest_list_size: i64,
    pub total_active_lists: i64,

    // Performance Metrics
    pub average_delivery_time: f64,
    pub bounce_rate: f64,
    pub complaint_rate: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CountryEngagement {
    pub country: String,
    pub total_opens: i64,
    pub unique_subscribers: i64,
    pub engagement_rate: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CityEngagement {
    pub city: String,
    pub country: String,
    pub total_opens: i64,
    pub unique_subscribers: i64,
    pub engagement_rate: f64,
} 