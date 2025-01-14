use sqlx::PgPool;
use uuid::Uuid;


use crate::{
    models::campaign::{Campaign, CreateCampaignDto, DeleteCampaignDto , CampaignStatus, CampaignType, ContentType},
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
                type,
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
                type as "campaign_type!: CampaignType",
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
}