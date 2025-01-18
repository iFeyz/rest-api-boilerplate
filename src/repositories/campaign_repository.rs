use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, Utc};


use crate::{
    models::campaign::{Campaign,  CampaignStatus, CampaignType, CreateCampaignDto, DeleteCampaignDto, PaginationParams, CampaignFilter, UpdateCampaignDto , CampaignResponse},
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
pub async fn find_all(&self, filter: Option<CampaignFilter>, pagination: Option<PaginationParams>) -> Result<CampaignResponse<Campaign>, ApiError> {
    let pagination = pagination.unwrap_or_default();
    let offset = (pagination.page.unwrap_or_else(|| 1) - 1) * pagination.per_page.unwrap_or_else(|| 10);

    // Préparer la liste des conditions et des paramètres dynamiques
    let mut conditions = Vec::new();
    let mut date_conditions = Vec::new();
    let mut params = Vec::new();

    if let Some(filter) = filter {
        if let Some(id) = filter.id {
            conditions.push(format!("id = {}", id));
            params.push(id.to_string());
        }

        if let Some(uuid) = filter.uuid {
            conditions.push(format!("uuid = {}", uuid));
            params.push(uuid.to_string());
        }

        if let Some(name) = filter.name {
            conditions.push(format!("name = {}", name));
            params.push(name);
        }
    // Adding for enums
    //    if let Some(status) = filter.status {
    //        conditions.push(format!("status = {}", status));
    //        params.push(status.to_string());
    //    }

        if let Some(subject) = filter.subject {
            conditions.push(format!("subject = {}", subject));
            params.push(subject);
        }

        if let Some(from_email) = filter.from_email {
            conditions.push(format!("from_email = {}", from_email));
            params.push(from_email);
        }

       // if let Some(campaign_type) = filter.campaign_type {
       //     conditions.push(format!("campaign_type = {}", campaign_type));
       //     params.push(campaign_type.to_string());
       // }

       if let Some(tags) = filter.tags {
        conditions.push(format!("tags = {}", tags));
        params.push(tags);
       }

       if let Some(messenger) = filter.messenger {
        conditions.push(format!("messenger = {}", messenger));
        params.push(messenger);
       }

       if let Some(to_send) = filter.to_send {
        conditions.push(format!("to_send = {}", to_send));
        params.push(to_send.to_string());
       }

       if let Some(sent) = filter.sent {
        conditions.push(format!("sent = {}", sent));
        params.push(sent.to_string());
       }

       if let Some(max_subscriber_id) = filter.max_subscriber_id {
        conditions.push(format!("max_subscriber_id = {}", max_subscriber_id));
        params.push(max_subscriber_id.to_string());
       }

       if let Some(last_subscriber_id) = filter.last_subscriber_id {
        conditions.push(format!("last_subscriber_id = {}", last_subscriber_id));
        params.push(last_subscriber_id.to_string());
       }

       // Need adding data filter 
       if let Some(started_at) = filter.started_at {
        date_conditions.push(format!("started_at = {}", started_at));
        params.push(started_at.to_rfc3339());
       }


    }
        // Adding other closes

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };
        // Adding date filter
        let where_over_clause = if date_conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {} < created_at", date_conditions.join(" AND "))
        };

        // Construire la requete SQL
        let query = format!(
            "SELECT * FROM campaigns {} {} ORDER BY {} {} LIMIT {} OFFSET {}", 
            where_clause,
            where_over_clause,
            pagination.sort_by.unwrap_or_else(|| "id".to_string()),
            pagination.sort_order.unwrap_or_else(|| "ASC".to_string()),
            pagination.per_page.unwrap_or_else(|| 10),
            offset
        );

        let mut query_builder = sqlx::query_as::<_, Campaign>(&query);

        for param in params {
            query_builder = query_builder.bind(param);
        }

        query_builder = query_builder
            .bind(pagination.per_page.unwrap_or_else(|| 10) as i64)
            .bind(offset as i64);

       

        let campaigns = query_builder
            .fetch_all(&self.pool)
            .await?;

        Ok(CampaignResponse {
            items: campaigns,
            page: pagination.page.unwrap_or_else(|| 1),
            per_page: pagination.per_page.unwrap_or_else(|| 10),
        })
    
}

    pub async fn find_by_id(&self, id: i32) -> Result<Option<Campaign>, ApiError> {
        let campaign = sqlx::query_as!(
            Campaign,
            r#"
            SELECT 
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
            FROM campaigns WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(campaign)
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