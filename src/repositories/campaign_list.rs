use crate::{
    models::campaign_list::{CampaignList, CreateCampaignListDto, UpdateCampaignListDto, PaginationDto},
    error::ApiError,
};

pub struct CampaignListRepository {
    pool: PgPool,
}

impl CampaignListRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, dto: CreateCampaignListDto) -> Result<CampaignList, ApiError> {
        let campaign_list = sqlx::query_as!(
            CampaignList,
            r#"
            INSERT INTO campaign_lists (campaign_id, list_id)
            VALUES ($1, $2)
            RETURNING id, campaign_id, list_id
            "#,
            dto.campaign_id,
            dto.list_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(campaign_list)
    }

    pub async fn find_by_campaign_id_and_list_id(&self, campaign_id: i32, list_id: i32) -> Result<Option<CampaignList>, ApiError> {
        let campaign_list = sqlx::query_as!(
            CampaignList,
            r#"
            SELECT * FROM campaign_lists WHERE campaign_id = $1 AND list_id = $2
            "#,
            campaign_id,
            list_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(campaign_list)
    }

    pub async fn find_all(&self, query: PaginationDto) -> Result<Vec<CampaignList>, ApiError> {
            let offset = (query.page - 1) * query.per_page;

            let campaign_lists = match (query.campaign_id, query.list_id, &query.status) {
                (Some(campaign_id), None, None) => {
                    sqlx::query_as!(
                        CampaignList,
                        r#"
                        SELECT * FROM campaign_lists WHERE campaign_id = $1
                        "#,
                        campaign_id,
                        list_id,
                        query.per_page as i64,
                        offset as i64
                    )
                    .fetch_all(&self.pool)
                    .await?
                }
                (None, Some(list_id), None) => {
                    sqlx::query_as!(
                        CampaignList,
                        r#"
                        SELECT * FROM campaign_lists WHERE list_id = $1
                        "#,
                        list_id,
                        query.per_page as i64,
                        offset as i64
                    )
                    .fetch_all(&self.pool)
                    .await?
                }
                (None, None, Some(status)) => {
                    sqlx::query_as!(
                        CampaignList,
                        r#"
                        SELECT * FROM campaign_lists WHERE status = $1
                        "#,
                        status
                    )
                    .fetch_all(&self.pool)
                    .await?
                }
                _ => {
                    sqlx::query_as!(
                        CampaignList,
                        r#"
                        SELECT * FROM campaign_lists
                        "#,
                        query.per_page as i64,
                        offset as i64
                    )
                    .fetch_all(&self.pool)
                    .await?
                }
            }

        Ok(campaign_lists)
    }

    pub async fn update(&self, campaign_id: i32, list_id: i32, dto: UpdateCampaignListDto) -> Result<Option<CampaignList>, ApiError> {
        let campaign_list = sqlx::query_as!(
            CampaignList,
            r#"
            UPDATE campaign_lists SET status = $3 WHERE campaign_id = $1 AND list_id = $2
            "#,
            campaign_id,
            list_id,
            dto.status
        )
    }

    pub async fn delete(&self, campaign_id: i32, list_id: i32) -> Result<Option<()>, ApiError> {
        sqlx::query!(
            r#"
            DELETE FROM campaign_lists WHERE campaign_id = $1 AND list_id = $2
            "#,
            campaign_id,
            list_id
        )
        .execute(&self.pool)
        .await?;
    }
}


