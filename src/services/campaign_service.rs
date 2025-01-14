use uuid::Uuid;
use serde_json::Value;

use crate::{
    error::ApiError,
    models::campaign::{Campaign, CreateCampaignDto, DeleteCampaignDto, PaginationDto, UpdateCampaignDto},
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

    pub async fn get_campaigns(&self, dto: PaginationDto) -> Result<Option<Vec<Campaign>>, ApiError> {
        println!("Getting campaigns: {:?}", dto);
        self.repository.find_all(dto).await
    }

    pub async fn update_campaign(&self, dto: UpdateCampaignDto) -> Result<Campaign, ApiError> {
        println!("Updating campaign: {:?}", dto);
        self.repository.update(dto).await
    }

    pub async fn delete_campaign(&self, dto: DeleteCampaignDto) -> Result<(), ApiError> {
        println!("Deleting campaign: {:?}", dto);
        self.repository.delete(dto).await
    }
}