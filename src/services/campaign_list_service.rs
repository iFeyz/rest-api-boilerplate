use crate::{
    error::ApiError,
    models::campaign_list::{CampaignList, CreateCampaignListDto, UpdateCampaignListDto, PaginationDto},
    repositories::campaign_list_repository::CampaignListRepository
};

pub struct CampaignListService {
    repository: CampaignListRepository
}

impl CampaignListService {
    pub fn new(repository: CampaignListRepository) -> Self {
        Self { repository }
    }

    pub async fn create_campaign_list(&self, campaign_list: CreateCampaignListDto) -> Result<CampaignList, ApiError> {
        self.repository.create(campaign_list).await
    }

    pub async fn get_campaign_list(&self, campaign_id: i32, list_id: i32) -> Result<Option<CampaignList>, ApiError> {
        self.repository.find_by_campaign_and_list(campaign_id, list_id).await
    }

    pub async fn get_campaign_lists(&self, campaign_id: i32) -> Result<Vec<CampaignList>, ApiError> {
        self.repository.get_campaign_lists(campaign_id).await
    }

    pub async fn update_campaign_list(&self, campaign_id: i32, list_id: i32, campaign_list: UpdateCampaignListDto) -> Result<Option<CampaignList>, ApiError> {
        self.repository.update(campaign_id, list_id, campaign_list).await
    }

    pub async fn delete_campaign_list(&self, campaign_id: i32, list_id: i32) -> Result<Option<()>, ApiError> {
        self.repository.delete(campaign_id, list_id).await
    }
}