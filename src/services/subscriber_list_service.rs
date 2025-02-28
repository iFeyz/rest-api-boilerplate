use crate::{
    models::subscriber_list::{SubscriberList, CreateSubscriberListDto, UpdateSubscriberListDto, PaginationDto},
    repositories::subscriber_list_repository::SubscriberListRepository,
    error::ApiError,
};

pub struct SubscriberListService {
    repository: SubscriberListRepository,
}

impl SubscriberListService {
    pub fn new(repository: SubscriberListRepository) -> Self {
        Self { repository }
    }

    pub async fn create_subscriber_list(&self, dto: CreateSubscriberListDto) -> Result<SubscriberList, ApiError> {
        self.repository.create(dto).await
    }

    pub async fn get_subscriber_list(&self, subscriber_id: i32, list_id: i32) -> Result<Option<SubscriberList>, ApiError> {
        self.repository.find_by_subscriber_id_and_list_id(subscriber_id, list_id).await
    }

    pub async fn find_all(&self, query: PaginationDto) -> Result<Vec<SubscriberList>, ApiError> {
        self.repository.find_all(&query).await
    }

    pub async fn update_subscriber_list(&self, subscriber_id: i32, list_id: i32, dto: UpdateSubscriberListDto) -> Result<Option<SubscriberList>, ApiError> {
        self.repository.update(subscriber_id, list_id, dto).await
    }

    pub async fn delete_subscriber_list(&self, subscriber_id: i32, list_id: i32) -> Result<Option<()>, ApiError> {
        self.repository.delete(subscriber_id, list_id).await
    }
}
