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

pub struct EmailViewsRepository {
    pool: PgPool,
}

use chrono::Utc;


impl EmailViewsRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, dto: CreateEmailViewDto) -> Result<EmailView, ApiError> {
        let email_view = sqlx::query_as!(
            EmailView,
            r#"
            INSERT INTO email_views (sequence_email_id, subscriber_id, ip_address, user_agent, country, city, region, latitude, longitude, metadata)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING *
            "#,
            dto.sequence_email_id,
            dto.subscriber_id,
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
        .await?;
        Ok(email_view)
    }
}