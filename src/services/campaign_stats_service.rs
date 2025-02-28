use actix_web::web;
use crate::repositories::campaign_stats_repository::CampaignStatsRepository;
use crate::models::campaign_stats::{CampaignStats, SequenceEmailDetailedStats};
use crate::error::ApiError;

pub struct CampaignStatsService {
    repository: web::Data<CampaignStatsRepository>,
}

impl CampaignStatsService {
    pub fn new(repository: web::Data<CampaignStatsRepository>) -> Self {
        Self { repository }
    }

    pub async fn get_campaign_stats(&self, campaign_id: i32) -> Result<CampaignStats, ApiError> {
        self.repository.get_campaign_stats(campaign_id).await
    }

    pub async fn get_campaign_detailed_stats(&self, campaign_id: i32) -> Result<CampaignStats, ApiError> {
        self.repository.get_campaign_detailed_stats(campaign_id).await
    }

    pub async fn get_sequence_email_stats(
        &self,
        campaign_id: i32,
        sequence_id: i32
    ) -> Result<SequenceEmailDetailedStats, ApiError> {
        self.repository.get_sequence_email_stats(campaign_id, sequence_id).await
    }
} 