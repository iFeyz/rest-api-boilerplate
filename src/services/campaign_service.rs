use uuid::Uuid;
use serde_json::Value;

use crate::{
    error::ApiError,
    models::campaign::{Campaign, CreateCampaignDto, DeleteCampaignDto},
    repositories::campaign_repository::CampaignRepository
};

pub struct CampaignService {
    repository: CampaignRepository
}

impl CampaignService {
    pub fn new(repository: CampaignRepository) -> Self {
        Self { repository }
    }

    pub async fn create_campaign(&self, dto: CreateCampaignDto) -> Result<Campaign, ApiError> {
        println!("Creating campaign: {:?}", dto);
        self.repository.create(dto).await
    }
}