use sqlx::PgPool;
use chrono::{DateTime, Utc};
use crate::models::subscriber_sequence_progress::{
    SubscriberSequenceProgress, 
    CreateSequenceProgressDto, 
    UpdateSequenceProgressDto
};
use crate::error::ApiError;

pub struct SubscriberSequenceProgressRepository {
    pool: PgPool,
}

impl SubscriberSequenceProgressRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self, 
        dto: CreateSequenceProgressDto
    ) -> Result<SubscriberSequenceProgress, ApiError> {
        let progress = sqlx::query_as!(
            SubscriberSequenceProgress,
            r#"
            INSERT INTO subscriber_sequence_progress 
            (subscriber_id, campaign_id, list_id, joined_at, current_position, completed)
            VALUES ($1, $2, $3, NOW(), 0, false)
            RETURNING 
                id as "id!: i32",
                subscriber_id as "subscriber_id!: i32",
                campaign_id as "campaign_id!: i32",
                list_id as "list_id!: i32",
                joined_at as "joined_at!: DateTime<Utc>",
                current_position as "current_position!: i32",
                last_email_sent_at as "last_email_sent_at?: DateTime<Utc>",
                next_email_scheduled_at as "next_email_scheduled_at?: DateTime<Utc>",
                completed as "completed!: bool",
                created_at as "created_at?: DateTime<Utc>",
                updated_at as "updated_at?: DateTime<Utc>"
            "#,
            dto.subscriber_id, dto.campaign_id, dto.list_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(progress)
    }

    pub async fn update(
        &self, 
        id: i32, 
        dto: UpdateSequenceProgressDto
    ) -> Result<SubscriberSequenceProgress, ApiError> {
        let progress = sqlx::query_as!(
            SubscriberSequenceProgress,
            r#"
            UPDATE subscriber_sequence_progress
            SET 
                current_position = COALESCE($1, current_position),
                last_email_sent_at = COALESCE($2, last_email_sent_at),
                next_email_scheduled_at = $3,
                completed = COALESCE($4, completed),
                updated_at = NOW()
            WHERE id = $5
            RETURNING 
                id as "id!: i32",
                subscriber_id as "subscriber_id!: i32",
                campaign_id as "campaign_id!: i32",
                list_id as "list_id!: i32",
                joined_at as "joined_at!: DateTime<Utc>",
                current_position as "current_position!: i32",
                last_email_sent_at as "last_email_sent_at?: DateTime<Utc>",
                next_email_scheduled_at as "next_email_scheduled_at?: DateTime<Utc>",
                completed as "completed!: bool",
                created_at as "created_at?: DateTime<Utc>",
                updated_at as "updated_at?: DateTime<Utc>"
            "#,
            dto.current_position,
            dto.last_email_sent_at,
            dto.next_email_scheduled_at, // Peut être NULL pour réinitialiser
            dto.completed,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(progress)
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<SubscriberSequenceProgress>, ApiError> {
        let progress = sqlx::query_as!(
            SubscriberSequenceProgress,
            r#"
            SELECT 
                id as "id!: i32",
                subscriber_id as "subscriber_id!: i32",
                campaign_id as "campaign_id!: i32",
                list_id as "list_id!: i32",
                joined_at as "joined_at!: DateTime<Utc>",
                current_position as "current_position!: i32",
                last_email_sent_at as "last_email_sent_at?: DateTime<Utc>",
                next_email_scheduled_at as "next_email_scheduled_at?: DateTime<Utc>",
                completed as "completed!: bool",
                created_at as "created_at?: DateTime<Utc>",
                updated_at as "updated_at?: DateTime<Utc>"
            FROM subscriber_sequence_progress
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(progress)
    }

    pub async fn find_by_subscriber_and_campaign(
        &self, 
        subscriber_id: i32, 
        campaign_id: i32
    ) -> Result<Option<SubscriberSequenceProgress>, ApiError> {
        let progress = sqlx::query_as!(
            SubscriberSequenceProgress,
            r#"
            SELECT 
                id as "id!: i32",
                subscriber_id as "subscriber_id!: i32",
                campaign_id as "campaign_id!: i32",
                list_id as "list_id!: i32",
                joined_at as "joined_at!: DateTime<Utc>",
                current_position as "current_position!: i32",
                last_email_sent_at as "last_email_sent_at?: DateTime<Utc>",
                next_email_scheduled_at as "next_email_scheduled_at?: DateTime<Utc>",
                completed as "completed!: bool",
                created_at as "created_at?: DateTime<Utc>",
                updated_at as "updated_at?: DateTime<Utc>"
            FROM subscriber_sequence_progress
            WHERE subscriber_id = $1 AND campaign_id = $2
            "#,
            subscriber_id, campaign_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(progress)
    }

    pub async fn find_pending_sends(&self) -> Result<Vec<SubscriberSequenceProgress>, ApiError> {
        let now = Utc::now();

        let progress_list = sqlx::query_as!(
            SubscriberSequenceProgress,
            r#"
            SELECT 
                id as "id!: i32",
                subscriber_id as "subscriber_id!: i32",
                campaign_id as "campaign_id!: i32",
                list_id as "list_id!: i32",
                joined_at as "joined_at!: DateTime<Utc>",
                current_position as "current_position!: i32",
                last_email_sent_at as "last_email_sent_at?: DateTime<Utc>",
                next_email_scheduled_at as "next_email_scheduled_at?: DateTime<Utc>",
                completed as "completed!: bool",
                created_at as "created_at?: DateTime<Utc>",
                updated_at as "updated_at?: DateTime<Utc>"
            FROM subscriber_sequence_progress
            WHERE completed = false 
            AND next_email_scheduled_at IS NOT NULL 
            AND next_email_scheduled_at <= $1
            ORDER BY next_email_scheduled_at ASC
            LIMIT 100
            "#,
            now
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(progress_list)
    }

    pub async fn delete(&self, id: i32) -> Result<Option<SubscriberSequenceProgress>, ApiError> {
        let progress = sqlx::query_as!(
            SubscriberSequenceProgress,
            r#"
            DELETE FROM subscriber_sequence_progress
            WHERE id = $1
            RETURNING 
                id as "id!: i32",
                subscriber_id as "subscriber_id!: i32",
                campaign_id as "campaign_id!: i32",
                list_id as "list_id!: i32",
                joined_at as "joined_at!: DateTime<Utc>",
                current_position as "current_position!: i32",
                last_email_sent_at as "last_email_sent_at?: DateTime<Utc>",
                next_email_scheduled_at as "next_email_scheduled_at?: DateTime<Utc>",
                completed as "completed!: bool",
                created_at as "created_at?: DateTime<Utc>",
                updated_at as "updated_at?: DateTime<Utc>"
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(progress)
    }
}