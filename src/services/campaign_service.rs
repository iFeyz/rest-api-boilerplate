use uuid::Uuid;
use serde_json::Value;

use crate::{
    error::ApiError,
    models::campaign::{Campaign, CreateCampaignDto, DeleteCampaignDto,  CampaignResponse, PaginationParams, CampaignFilter, UpdateCampaignDto},
    repositories::campaign_repository::CampaignRepository
};

pub struct CampaignService {
    repository: CampaignRepository
}

impl CampaignService {
    pub fn new(repository: CampaignRepository) -> Self {
        Self { repository }
    }

    pub async fn get_campaign(&self, id: i32) -> Result<Option<Campaign>, ApiError> {
        println!("Getting campaign: {:?}", id);
        self.repository.find_by_id(id).await
    }

    pub async fn create_campaign(&self, dto: CreateCampaignDto) -> Result<Campaign, ApiError> {
        println!("Creating campaign: {:?}", dto);
        self.repository.create(dto).await
    }

    pub async fn get_campaigns(&self, filter: Option<CampaignFilter>, pagination: Option<PaginationParams>) -> Result<CampaignResponse<Campaign>, ApiError> {
        println!("Getting campaigns: {:?}", filter);
        self.repository.find_all(filter, pagination).await
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