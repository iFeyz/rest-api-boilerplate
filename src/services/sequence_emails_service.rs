use crate::{
    models::sequence_emails::{SequenceEmail, CreateSequenceEmailDto, UpdateSequenceEmailDto, DeleteSequenceEmailDto, PaginationDto},
    repositories::sequence_email_repository::SequenceEmailRepository,
    error::ApiError,
};

pub struct SequenceEmailService {
    repository: SequenceEmailRepository,
}

impl SequenceEmailService {
    pub fn new(repository: SequenceEmailRepository) -> Self {
        Self { repository }
    }

    pub async fn create_sequence_email(&self, dto: CreateSequenceEmailDto) -> Result<SequenceEmail, ApiError> {
        self.repository.create(dto).await
    }

    pub async fn find_all(&self, dto: PaginationDto) -> Result<Vec<SequenceEmail>, ApiError> {
        println!("Finding all sequence emails campaign_id: {}", dto.to_string());
        self.repository.find_all(dto).await
    }

    pub async fn update_sequence_email(&self, id: i32, dto: UpdateSequenceEmailDto) -> Result<Option<SequenceEmail>, ApiError> {
        self.repository.update(id, dto).await
    }

    pub async fn delete_sequence_email(&self, id: i32) -> Result<Option<()>, ApiError> {
        self.repository.delete(id).await
    }
}
