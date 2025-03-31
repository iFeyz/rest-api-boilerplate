use crate::{
    models::sequence_email::{SequenceEmail, CreateSequenceEmailDto, UpdateSequenceEmailDto, PaginationDto},
    repositories::sequence_email_repository::SequenceEmailRepository,
    error::ApiError,
};
use chrono::{DateTime, Utc};
use serde_json::Value as JsonValue;

pub struct SequenceEmailService {
    repository: SequenceEmailRepository,
}

impl SequenceEmailService {
    pub fn new(repository: SequenceEmailRepository) -> Self {
        Self { repository }
    }

    pub async fn create_sequence_email(&self, dto: CreateSequenceEmailDto) -> Result<SequenceEmail, ApiError> {
        // Default values for delay fields if not provided
        let dto_with_defaults = CreateSequenceEmailDto {
            delay_type: if dto.delay_type.is_empty() { "absolute".to_string() } else { dto.delay_type },
            delay_value: dto.delay_value,
            delay_unit: dto.delay_unit,
            ..dto
        };
        
        self.repository.create(dto_with_defaults).await
    }

    pub async fn find_all(&self, pagination: PaginationDto) -> Result<Vec<SequenceEmail>, ApiError> {
        self.repository.find_all(pagination).await
    }

    pub async fn update_sequence_email(&self, id: i32, dto: UpdateSequenceEmailDto) -> Result<Option<SequenceEmail>, ApiError> {
        self.repository.update(id, dto).await
    }

    pub async fn delete_sequence_email(&self, id: i32) -> Result<Option<()>, ApiError> {
        self.repository.delete(id).await
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<SequenceEmail>, ApiError> {
        // We'll use the repository's method instead of direct query
        let result = sqlx::query_as!(
            SequenceEmail,
            r#"
            SELECT 
                id as "id!: i32",
                campaign_id as "campaign_id!: i32",
                position as "position!: i32",
                subject as "subject!: String",
                body as "body!: String",
                template_id as "template_id?: i32",
                content_type as "content_type!: _",
                status as "status!: _",
                metadata as "metadata!: JsonValue",
                is_active as "is_active!: bool",
                send_at as "send_at?: DateTime<Utc>",
                delay_type as "delay_type!: String",
                delay_value as "delay_value?: i32",
                delay_unit as "delay_unit?: String",
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>"
            FROM sequence_emails
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(self.repository.pool())
        .await
        .map_err(ApiError::DatabaseError)?;

        Ok(result)
    }

    pub async fn find_by_campaign_id(&self, campaign_id: i32) -> Result<Vec<SequenceEmail>, ApiError> {
        self.repository.find_by_campaign_id(campaign_id).await
    }

    pub async fn get_active_sequence_emails(&self, campaign_id: i32) -> Result<Vec<SequenceEmail>, ApiError> {
        // Modified implementation using get_active_sequence_email
        let active_email = self.repository.get_active_sequence_email(campaign_id).await?;
        
        if let Some(email) = active_email {
            Ok(vec![email])
        } else {
            Ok(vec![])
        }
    }

    pub async fn get_pending_sequence_emails(&self) -> Result<Vec<SequenceEmail>, ApiError> {
        self.repository.get_pending_sequence_emails().await
    }
} 