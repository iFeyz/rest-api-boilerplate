use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, Utc};


use crate::{
    models::campaign::{Campaign,  CampaignStatus, CampaignType, CreateCampaignDto, DeleteCampaignDto, PaginationDto, UpdateCampaignDto},
    error::ApiError,
};


pub struct CampaignRepository {
    pool: PgPool,
}

impl CampaignRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, campaign: CreateCampaignDto) -> Result<Campaign, ApiError> {
        let default_meta = serde_json::json!({});
        let campaign = sqlx::query_as!(
            Campaign,
            r#"
            INSERT INTO campaigns (
                name, 
                uuid,
                subject, 
                from_email, 
                status, 
                campaign_type,
                tags, 
                messenger,
                headers,
                to_send,
                sent,
                max_subscriber_id,
                last_subscriber_id,
                archive,
                archive_slug,
                archive_template_id,
                archive_meta,
                sequence_start_date,  
                sequence_end_date    
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19)
            RETURNING 
                id, uuid, 
                name as "name!", 
                subject as "subject!", 
                from_email as "from_email!", 
                status as "status!: CampaignStatus",
                campaign_type as "campaign_type!: CampaignType",
                tags,
                messenger as "messenger!",
                headers as "headers!",
                to_send as "to_send!",
                sent as "sent!",
                max_subscriber_id as "max_subscriber_id!",
                last_subscriber_id as "last_subscriber_id!",
                archive as "archive!",
                archive_slug,
                archive_template_id,
                archive_meta as "archive_meta!",
                started_at,
                created_at,
                updated_at,
                sequence_start_date,
                sequence_end_date
            "#,
            
            campaign.name,
            Uuid::new_v4(),
            campaign.subject,
            campaign.from_email,
            campaign.status as CampaignStatus,
            campaign.campaign_type as CampaignType,
            &campaign.tags.unwrap_or_default(),
            campaign.messenger,
            campaign.headers,
            0i32,  // to_send
            0i32,  // sent
            0i32,  // max_subscriber_id
            0i32,  // last_subscriber_id
            false, // archive
            Option::<String>::None, // archive_slug
            Option::<i32>::None,    // archive_template_id
            default_meta,           // archive_meta
            None::<DateTime<Utc>>,  // sequence_start_date
            None::<DateTime<Utc>>   // sequence_end_date
        )
        .fetch_one(&self.pool)
        .await?;
    
        Ok(campaign)
    }
pub async fn find_all(&self, dto: PaginationDto) -> Result<Option<Vec<Campaign>>, ApiError> {
    let mut query = String::from(
        r#"
        SELECT 
            id, uuid, 
            name, 
            subject, 
            from_email, 
            status::campaign_status as "status",
            campaign_type::campaign_type as "campaign_type",
            tags,
            messenger,
            headers,
            to_send,
            sent,
            max_subscriber_id,
            last_subscriber_id,
            archive,
            archive_slug,
            archive_template_id,
            archive_meta,
            started_at,
            created_at,
            updated_at,
            sequence_start_date,
            sequence_end_date
        FROM campaigns
        "#
    );

    let mut conditions: Vec<String> = Vec::new();
    let mut where_clause = String::new();

    // Build WHERE conditions
    if let Some(status) = &dto.status {
        where_clause.push_str(" WHERE status::campaign_status = $1");
    }

    if let Some(campaign_type) = &dto.campaign_type {
        if where_clause.is_empty() {
            where_clause.push_str(" WHERE campaign_type::campaign_type = $1");
        } else {
            where_clause.push_str(" AND campaign_type::campaign_type = $2");
        }
    }

    if let Some(messenger) = &dto.messenger {
        if where_clause.is_empty() {
            where_clause.push_str(" WHERE messenger = $1");
        } else if where_clause.contains("$2") {
            where_clause.push_str(" AND messenger = $3");
        } else {
            where_clause.push_str(" AND messenger = $2");
        }
    }

    query.push_str(&where_clause);

    // Add ORDER BY
    query.push_str(&format!(" ORDER BY {} {}", 
        if ["name", "status", "created_at", "updated_at"].contains(&dto.order_by.as_str()) {
            &dto.order_by
        } else {
            "created_at"
        },
        if ["ASC", "DESC"].contains(&dto.order.to_uppercase().as_str()) {
            dto.order.to_uppercase()
        } else {
            "DESC".to_string()
        }
    ));

    // Add pagination
    let offset = (dto.page - 1) * dto.per_page;
    query.push_str(&format!(" LIMIT {} OFFSET {}", dto.per_page, offset));

    // Create and execute query with proper bindings
    let mut db_query = sqlx::query_as::<_, Campaign>(&query);

    // Bind parameters in correct order
    if let Some(status) = &dto.status {
        db_query = db_query.bind(status);
    }
    if let Some(campaign_type) = &dto.campaign_type {
        db_query = db_query.bind(campaign_type);
    }
    if let Some(messenger) = &dto.messenger {
        db_query = db_query.bind(messenger);
    }

    let campaigns = db_query.fetch_all(&self.pool).await?;

    if campaigns.is_empty() {
        Ok(None)
    } else {
        Ok(Some(campaigns))
    }
}

    pub async fn update(&self, campaign: UpdateCampaignDto) -> Result<Campaign, ApiError> {
        let campaign = sqlx::query_as!(
            Campaign,
            r#"
            UPDATE campaigns
            SET 
                name = CASE WHEN $1::text IS NOT NULL THEN $1 ELSE name END,
                subject = CASE WHEN $2::text IS NOT NULL THEN $2 ELSE subject END,
                from_email = CASE WHEN $3::text IS NOT NULL THEN $3 ELSE from_email END,
                status = CASE WHEN $4::campaign_status IS NOT NULL THEN $4 ELSE status END,
                campaign_type = CASE WHEN $5::campaign_type IS NOT NULL THEN $5 ELSE campaign_type END,
                tags = CASE WHEN $6::text[] IS NOT NULL THEN $6 ELSE tags END,
                messenger = CASE WHEN $7::text IS NOT NULL THEN $7 ELSE messenger END,
                headers = CASE WHEN $8::jsonb IS NOT NULL THEN $8 ELSE headers END,
                to_send = CASE WHEN $9::int4 IS NOT NULL THEN $9 ELSE to_send END,
                sent = CASE WHEN $10::int4 IS NOT NULL THEN $10 ELSE sent END,
                max_subscriber_id = CASE WHEN $11::int4 IS NOT NULL THEN $11 ELSE max_subscriber_id END,
                last_subscriber_id = CASE WHEN $12::int4 IS NOT NULL THEN $12 ELSE last_subscriber_id END,
                archive = CASE WHEN $13::boolean IS NOT NULL THEN $13 ELSE archive END,
                archive_slug = CASE WHEN $14::text IS NOT NULL THEN $14 ELSE archive_slug END,
                archive_template_id = CASE WHEN $15::int4 IS NOT NULL THEN $15 ELSE archive_template_id END,
                archive_meta = CASE WHEN $16::jsonb IS NOT NULL THEN $16 ELSE archive_meta END,
                sequence_start_date = CASE WHEN $17::timestamptz IS NOT NULL THEN $17 ELSE sequence_start_date END,
                sequence_end_date = CASE WHEN $18::timestamptz IS NOT NULL THEN $18 ELSE sequence_end_date END
            WHERE id = $19
            RETURNING 
                id, uuid, 
                name as "name!", 
                subject as "subject!", 
                from_email as "from_email!", 
                status as "status!: CampaignStatus",
                campaign_type as "campaign_type!: CampaignType",
                tags,
                messenger as "messenger!",
                headers as "headers!",
                to_send as "to_send!",
                sent as "sent!",
                max_subscriber_id as "max_subscriber_id!",
                last_subscriber_id as "last_subscriber_id!",
                archive as "archive!",
                archive_slug,
                archive_template_id,
                archive_meta as "archive_meta!",
                started_at,
                created_at,
                updated_at,
                sequence_start_date,
                sequence_end_date
            "#,
            campaign.name.as_deref(),
            campaign.subject.as_deref(),
            campaign.from_email.as_deref(),
            campaign.status as Option<CampaignStatus>,
            campaign.campaign_type as Option<CampaignType>,
            campaign.tags.as_deref(),
            campaign.messenger.as_deref(),
            campaign.headers,
            campaign.to_send,
            campaign.sent,
            campaign.max_subscriber_id,
            campaign.last_subscriber_id,
            campaign.archive,
            campaign.archive_slug.as_deref(),
            campaign.archive_template_id,
            campaign.archive_meta,
            campaign.sequence_start_date,
            campaign.sequence_end_date,
            campaign.id
        )
        .fetch_one(&self.pool)
        .await?;
    
        Ok(campaign)
    }

    pub async fn delete(&self, campaign: DeleteCampaignDto) -> Result<(), ApiError> {
        let result = if let Some(id) = campaign.id {
            sqlx::query!("DELETE FROM campaigns WHERE id = $1", id)
                .execute(&self.pool)
                .await?
        } else if let Some(uuid) = campaign.uuid {
            sqlx::query!("DELETE FROM campaigns WHERE uuid = $1", uuid)
                .execute(&self.pool)
                .await?
        } else {
            return Err(ApiError::NotFound);
        };

        if result.rows_affected() == 0 {
            return Err(ApiError::NotFound);
        }

        Ok(())
    }
}