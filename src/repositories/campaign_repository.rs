use sqlx::PgPool;
use uuid::Uuid;


use crate::{
    models::campaign::{Campaign, CreateCampaignDto, DeleteCampaignDto , CampaignStatus, CampaignType, ContentType, PaginationDto, UpdateCampaignDto},
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
                body, 
                altbody, 
                content_type, 
                send_at, 
                status, 
                campaign_type,
                tags, 
                messenger,
                template_id,
                headers,
                to_send,
                sent,
                max_subscriber_id,
                last_subscriber_id,
                archive,
                archive_slug,
                archive_template_id,
                archive_meta
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22)
            RETURNING 
                id, uuid, 
                name as "name!", 
                subject as "subject!", 
                from_email as "from_email!", 
                body as "body!", 
                altbody as "altbody!", 
                content_type as "content_type!: ContentType",
                send_at,
                status as "status!: CampaignStatus",
                campaign_type as "campaign_type!: CampaignType",
                tags,
                messenger as "messenger!",
                template_id,
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
                updated_at
            "#,
            campaign.name,
            Uuid::new_v4(),
            campaign.subject,
            campaign.from_email,
            campaign.body,
            campaign.altbody,
            campaign.content_type as ContentType,
            campaign.send_at,
            campaign.status as CampaignStatus,
            campaign.campaign_type as CampaignType,
            &campaign.tags.unwrap_or_default(),
            campaign.messenger,
            campaign.template_id,
            campaign.headers,
            0i32,  // to_send
            0i32,  // sent
            0i32,  // max_subscriber_id
            0i32,  // last_subscriber_id
            false, // archive
            Option::<String>::None, // archive_slug
            Option::<i32>::None,    // archive_template_id
            default_meta            // archive_meta
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
                body, 
                altbody, 
                content_type,
                send_at,
                status::campaign_status as "status",
                campaign_type::campaign_type as "campaign_type",
                tags,
                messenger,
                template_id,
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
                updated_at
            FROM campaigns
            "#
        );

        let mut conditions = Vec::new();

        // Build WHERE conditions
        if let Some(search_query) = &dto.query {
            conditions.push("name ILIKE '%' || $1 || '%'");
        }

        if let Some(ids) = &dto.id {
            conditions.push("id = ANY($2)");
        }

        if let Some(status) = &dto.status {
            conditions.push("status::campaign_status = $3");
        }

        if let Some(campaign_type) = &dto.campaign_type {
            conditions.push("type::campaign_type = $4");
        }

        if let Some(tags) = &dto.tags {
            conditions.push("tags = $5");
        }

        if let Some(messenger) = &dto.messenger {
            conditions.push("messenger = $6");
        }

        // Add WHERE clause if there are conditions
        if !conditions.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&conditions.join(" AND "));
        }

        // Add ORDER BY
        let order_by = match dto.order_by.as_str() {
            "name" | "status" | "created_at" | "updated_at" => &dto.order_by,
            _ => "created_at"
        };
        let order = match dto.order.to_uppercase().as_str() {
            "ASC" | "DESC" => dto.order.to_uppercase(),
            _ => "DESC".to_string()
        };
        query.push_str(&format!(" ORDER BY {} {}", order_by, order));

        // Add pagination
        let offset = (dto.page - 1) * dto.per_page;
        query.push_str(&format!(" LIMIT {} OFFSET {}", dto.per_page, offset));

        // Execute the query with proper bindings
        let mut db_query = sqlx::query_as::<_, Campaign>(&query);

        if let Some(search_query) = &dto.query {
            db_query = db_query.bind(search_query);
        }
        if let Some(ids) = &dto.id {
            db_query = db_query.bind(ids);
        }
        if let Some(status) = &dto.status {
            db_query = db_query.bind(status);
        }
        if let Some(campaign_type) = &dto.campaign_type {
            db_query = db_query.bind(campaign_type);
        }
        if let Some(tags) = &dto.tags {
            db_query = db_query.bind(tags);
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
                body = CASE WHEN $4::text IS NOT NULL THEN $4 ELSE body END,
                altbody = CASE WHEN $5::text IS NOT NULL THEN $5 ELSE altbody END,
                content_type = CASE WHEN $6::text IS NOT NULL THEN $6::content_type ELSE content_type END,
                send_at = CASE WHEN $7::timestamptz IS NOT NULL THEN $7 ELSE send_at END,
                status = CASE WHEN $8::text IS NOT NULL THEN $8::campaign_status ELSE status END,
                campaign_type = CASE WHEN $9::text IS NOT NULL THEN $9::campaign_type ELSE campaign_type END,
                tags = CASE WHEN $10::text[] IS NOT NULL THEN $10 ELSE tags END,
                messenger = CASE WHEN $11::text IS NOT NULL THEN $11 ELSE messenger END,
                template_id = CASE WHEN $12::int4 IS NOT NULL THEN $12 ELSE template_id END,
                headers = CASE WHEN $13::jsonb IS NOT NULL THEN $13 ELSE headers END,
                to_send = CASE WHEN $14::int4 IS NOT NULL THEN $14 ELSE to_send END,
                sent = CASE WHEN $15::int4 IS NOT NULL THEN $15 ELSE sent END,
                max_subscriber_id = CASE WHEN $16::int4 IS NOT NULL THEN $16 ELSE max_subscriber_id END,
                last_subscriber_id = CASE WHEN $17::int4 IS NOT NULL THEN $17 ELSE last_subscriber_id END,
                archive = CASE WHEN $18::boolean IS NOT NULL THEN $18 ELSE archive END,
                archive_slug = CASE WHEN $19::text IS NOT NULL THEN $19 ELSE archive_slug END,
                archive_template_id = CASE WHEN $20::int4 IS NOT NULL THEN $20 ELSE archive_template_id END,
                archive_meta = CASE WHEN $21::jsonb IS NOT NULL THEN $21 ELSE archive_meta END
            WHERE id = $22
            RETURNING 
                id, uuid, 
                name as "name!", 
                subject as "subject!", 
                from_email as "from_email!", 
                body as "body!", 
                altbody as "altbody!", 
                content_type::text as "content_type!: ContentType",
                send_at,
                status::text as "status!: CampaignStatus",
                campaign_type::text as "campaign_type!: CampaignType",
                tags,
                messenger as "messenger!",
                template_id,
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
                updated_at
            "#,
            campaign.name.as_deref(),
            campaign.subject.as_deref(),
            campaign.from_email.as_deref(),
            campaign.body.as_deref(),
            campaign.altbody.as_deref(),
            campaign.content_type.map(|ct| ct.to_string()),
            campaign.send_at,
            campaign.status.map(|s| s.to_string()),
            campaign.campaign_type.map(|ct| ct.to_string()),
            campaign.tags.as_deref(),
            campaign.messenger.as_deref(),
            campaign.template_id,
            campaign.headers,
            campaign.to_send,
            campaign.sent,
            campaign.max_subscriber_id,
            campaign.last_subscriber_id,
            campaign.archive,
            campaign.archive_slug,
            campaign.archive_template_id,
            campaign.archive_meta,
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