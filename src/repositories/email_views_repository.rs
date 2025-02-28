use sqlx::PgPool;
use crate::{
    models::email_views::{
        EmailView,
        CreateEmailViewDto,
        GetEmailViewDto,
        PaginationDto
    },
    error::ApiError
};

#[derive(Clone)]
pub struct EmailViewsRepository {
    pool: PgPool,
}

use chrono::Utc;


impl EmailViewsRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, dto: CreateEmailViewDto) -> Result<EmailView, ApiError> {
        tracing::info!("Creating email view with data: {:?}", dto);
        
        let email_view = sqlx::query_as!(
            EmailView,
            r#"
            INSERT INTO email_views (
                sequence_email_id, 
                subscriber_id, 
                campaign_id, 
                ip_address, 
                user_agent, 
                country, 
                city, 
                region, 
                latitude, 
                longitude, 
                metadata
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            ON CONFLICT (subscriber_id, sequence_email_id, campaign_id) 
            DO UPDATE SET 
                opened_at = NOW(),
                ip_address = EXCLUDED.ip_address,
                user_agent = EXCLUDED.user_agent
            RETURNING *
            "#,
            dto.sequence_email_id,
            dto.subscriber_id,
            dto.campaign_id,
            dto.ip_address,
            dto.user_agent,
            dto.country,
            dto.city,
            dto.region,
            dto.latitude,
            dto.longitude,
            dto.metadata.unwrap_or_default()
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Database error creating email view: {}", e);
            ApiError::DatabaseError(e)
        })?;

        tracing::info!("Successfully created email view: {:?}", email_view);
        Ok(email_view)
    }



}