use sqlx::PgPool;
use crate::{
    error::ApiError,
    models::campaign_list::{CampaignList, CreateCampaignListDto, UpdateCampaignListDto, PaginationDto}
};

pub struct CampaignListRepository {
    pool: PgPool
}

impl CampaignListRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, campaign_list: CreateCampaignListDto) -> Result<CampaignList, ApiError> {
        let result = sqlx::query_as!(
            CampaignList,
            r#"
            INSERT INTO campaign_lists (campaign_id, list_id, list_name)
            SELECT $1, $2, name 
            FROM lists 
            WHERE id = $2
            RETURNING id as "id!: i32", 
                      campaign_id as "campaign_id!: i32", 
                      list_id as "list_id!: i32",
                      list_name as "list_name!"
            "#,
            campaign_list.campaign_id,
            campaign_list.list_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    pub async fn find_by_campaign_and_list(&self, campaign_id: i32, list_id: i32) -> Result<Option<CampaignList>, ApiError> {
        let result = sqlx::query_as!(
            CampaignList,
            r#"
            SELECT id as "id!: i32",
                   campaign_id as "campaign_id!: i32",
                   list_id as "list_id!: i32",
                   list_name as "list_name!"
            FROM campaign_lists
            WHERE campaign_id = $1 AND list_id = $2
            "#,
            campaign_id,
            list_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }

    pub async fn find_all(&self, pagination: PaginationDto) -> Result<Vec<CampaignList>, ApiError> {
        let offset = (pagination.page - 1) * pagination.per_page;
        
        let result = sqlx::query_as!(
            CampaignList,
            r#"
            SELECT id as "id!: i32",
                   campaign_id as "campaign_id!: i32",
                   list_id as "list_id!: i32",
                   list_name as "list_name!"
            FROM campaign_lists
            ORDER BY id DESC
            LIMIT $1 OFFSET $2
            "#,
            pagination.per_page as i64,
            offset as i64
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(result)
    }

    pub async fn update(&self, campaign_id: i32, list_id: i32, campaign_list: UpdateCampaignListDto) -> Result<Option<CampaignList>, ApiError> {
        let result = sqlx::query_as!(
            CampaignList,
            r#"
            UPDATE campaign_lists
            SET list_name = CASE 
                WHEN $3::text IS NOT NULL THEN $3
                ELSE list_name
            END
            WHERE campaign_id = $1 AND list_id = $2
            RETURNING id as "id!: i32",
                      campaign_id as "campaign_id!: i32",
                      list_id as "list_id!: i32",
                      list_name as "list_name!"
            "#,
            campaign_id,
            list_id,
            campaign_list.list_name
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }

    pub async fn delete(&self, campaign_id: i32, list_id: i32) -> Result<Option<()>, ApiError> {
        let result = sqlx::query!(
            r#"
            DELETE FROM campaign_lists
            WHERE campaign_id = $1 AND list_id = $2
            "#,
            campaign_id,
            list_id
        )
        .execute(&self.pool)
        .await?;

        Ok(if result.rows_affected() > 0 { Some(()) } else { None })
    }
} 