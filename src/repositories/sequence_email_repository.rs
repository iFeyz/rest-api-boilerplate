use sqlx::PgPool;
use serde_json::Value as JsonValue;
use chrono::{DateTime, Utc};
use crate::{
    models::sequence_email::{
        SequenceEmail, 
        CreateSequenceEmailDto, 
        UpdateSequenceEmailDto, 
        PaginationDto,
        SequenceEmailStatus
    },
    error::ApiError,
};

#[derive(Clone)]
pub struct SequenceEmailRepository {
    pool: PgPool,
}

impl SequenceEmailRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, dto: CreateSequenceEmailDto) -> Result<SequenceEmail, ApiError> {
        let sequence_email = sqlx::query_as!(
            SequenceEmail,
            r#"
            INSERT INTO sequence_emails (
                campaign_id, 
                position, 
                subject, 
                body, 
                template_id, 
                content_type, 
                metadata, 
                is_active,
                send_at,
                status,
                created_at,
                updated_at
            )
            VALUES (
                $1, 
                $2, 
                $3, 
                $4, 
                $5, 
                $6::content_type, 
                $7, 
                $8, 
                $9, 
                'draft'::sequence_email_status,
                CURRENT_TIMESTAMP, 
                CURRENT_TIMESTAMP
            )
            RETURNING 
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
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>"
            "#,
            dto.campaign_id,
            dto.position,
            dto.subject,
            dto.body,
            dto.template_id,
            dto.content_type as _,
            dto.metadata,
            dto.is_active,
            dto.send_at
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(sequence_email)
    }

    pub async fn find_all(&self, dto: PaginationDto) -> Result<Vec<SequenceEmail>, ApiError> {
        let offset = (dto.page - 1) * dto.per_page;

        let sequence_emails = sqlx::query_as!(
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
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>"
            FROM sequence_emails
            WHERE campaign_id = $1
            ORDER BY id DESC
            LIMIT $2 OFFSET $3
            "#,
            dto.campaign_id,
            dto.per_page,
            offset
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(sequence_emails)
    }
    
    pub async fn update(&self, id: i32, dto: UpdateSequenceEmailDto) -> Result<Option<SequenceEmail>, ApiError> {
        let sequence_email = sqlx::query_as!(
            SequenceEmail,
            r#"
            UPDATE sequence_emails
            SET 
                subject = COALESCE($2, subject),
                body = COALESCE($3, body),
                template_id = COALESCE($4, template_id),
                content_type = COALESCE($5::content_type, content_type),
                metadata = COALESCE($6, metadata),
                is_active = COALESCE($7, is_active),
                send_at = $8,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $1
            RETURNING 
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
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>"
            "#,
            id,
            dto.subject,
            dto.body,
            dto.template_id,
            dto.content_type as _,
            dto.metadata,
            dto.is_active,
            dto.send_at
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(sequence_email)
    }

    pub async fn delete(&self, id: i32) -> Result<Option<()>, ApiError> {
        let result = sqlx::query!(
            r#"DELETE FROM sequence_emails WHERE id = $1"#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(if result.rows_affected() > 0 { Some(()) } else { None })
    }

    pub async fn get_active_sequence_email(&self, campaign_id: i32) -> Result<Option<SequenceEmail>, ApiError> {
        let sequence_email = sqlx::query_as!(
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
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>"
            FROM sequence_emails
            WHERE campaign_id = $1
            AND is_active = true
            AND (send_at IS NULL OR send_at <= NOW())
            ORDER BY position ASC
            LIMIT 1
            "#,
            campaign_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(sequence_email)
    }

    pub async fn get_pending_sequence_emails(&self) -> Result<Vec<SequenceEmail>, ApiError> {
        let sequence_emails = sqlx::query_as!(
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
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>"
            FROM sequence_emails
            WHERE is_active = true
            AND send_at <= NOW()
            AND status = 'draft'
            AND NOT EXISTS (
                SELECT 1 FROM email_views 
                WHERE email_views.sequence_email_id = sequence_emails.id
            )
            ORDER BY campaign_id, position ASC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(sequence_emails)
    }

    pub async fn update_status(&self, id: i32, new_status: SequenceEmailStatus) -> Result<(), ApiError> {
        sqlx::query!(
            r#"
            UPDATE sequence_emails 
            SET status = $1::sequence_email_status,
                updated_at = NOW()
            WHERE id = $2
            "#,
            new_status as _,
            id
        )
        .execute(&self.pool)
        .await
        .map_err(ApiError::DatabaseError)?;

        Ok(())
    }
} 