use sqlx::PgPool;
use crate::{
    models::subscriber_list::{
        SubscriberList, 
        CreateSubscriberListDto, 
        UpdateSubscriberListDto, 
        PaginationDto,
        SubscriptionStatus
    },
    error::ApiError,
};

pub struct SubscriberListRepository {
    pool: PgPool,
}

impl SubscriberListRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, dto: CreateSubscriberListDto) -> Result<SubscriberList, ApiError> {
        let subscriber_list = sqlx::query_as!(
            SubscriberList,
            r#"
            INSERT INTO subscriber_lists (subscriber_id, list_id, meta, status)
            VALUES ($1, $2, $3, $4)
            RETURNING 
                subscriber_id, 
                list_id,
                meta,
                status as "status: SubscriptionStatus",
                created_at,
                updated_at
            "#,
            dto.subscriber_id,
            dto.list_id,
            dto.meta,
            dto.status as SubscriptionStatus
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(subscriber_list)
    }

    pub async fn find_by_subscriber_id_and_list_id(&self, subscriber_id: i32, list_id: i32) -> Result<Option<SubscriberList>, ApiError> {
        let subscriber_list = sqlx::query_as!(
            SubscriberList,
            r#"
            SELECT 
                subscriber_id,
                list_id,
                meta,
                status as "status: SubscriptionStatus",
                created_at,
                updated_at
            FROM subscriber_lists 
            WHERE subscriber_id = $1 AND list_id = $2
            "#,
            subscriber_id,
            list_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(subscriber_list)
    }
    // TODO  NOT FINISHED YET
    pub async fn find_all(&self, query: &PaginationDto) -> Result<Vec<SubscriberList>, ApiError> {
        let offset = (query.page - 1) * query.per_page;
        
        let subscriber_lists = match (query.subscriber_id, query.list_id, &query.status) {
            (Some(subscriber_id), None, None) => {
                sqlx::query_as!(
                    SubscriberList,
                    r#"
                    SELECT 
                        subscriber_id,
                        list_id,
                        meta,
                        status as "status!: SubscriptionStatus",
                        created_at,
                        updated_at
                    FROM subscriber_lists 
                    WHERE subscriber_id = $1
                    ORDER BY created_at DESC
                    LIMIT $2 OFFSET $3
                    "#,
                    subscriber_id,
                    query.per_page as i64,
                    offset as i64
                )
                .fetch_all(&self.pool)
                .await?
            },
            (None, Some(list_id), None) => {
                sqlx::query_as!(
                    SubscriberList,
                    r#"
                    SELECT 
                        subscriber_id,
                        list_id,
                        meta,
                        status as "status!: SubscriptionStatus",
                        created_at,
                        updated_at
                    FROM subscriber_lists 
                    WHERE list_id = $1
                    ORDER BY created_at DESC
                    LIMIT $2 OFFSET $3
                    "#,
                    list_id,
                    query.per_page as i64,
                    offset as i64
                )
                .fetch_all(&self.pool)
                .await?
            },
            (Some(subscriber_id), Some(list_id), None) => {
                sqlx::query_as!(
                    SubscriberList,
                    r#"
                    SELECT 
                        subscriber_id,
                        list_id,
                        meta,
                        status as "status!: SubscriptionStatus",
                        created_at,
                        updated_at
                    FROM subscriber_lists 
                    WHERE subscriber_id = $1 AND list_id = $2
                    ORDER BY created_at DESC
                    LIMIT $3 OFFSET $4
                    "#,
                    subscriber_id,
                    list_id,
                    query.per_page as i64,
                    offset as i64
                )
                .fetch_all(&self.pool)
                .await?
            },
            _ => {
                sqlx::query_as!(
                    SubscriberList,
                    r#"
                    SELECT 
                        subscriber_id,
                        list_id,
                        meta,
                        status as "status!: SubscriptionStatus",
                        created_at,
                        updated_at
                    FROM subscriber_lists 
                    ORDER BY created_at DESC
                    LIMIT $1 OFFSET $2
                    "#,
                    query.per_page as i64,
                    offset as i64
                )
                .fetch_all(&self.pool)
                .await?
            }
        };

        Ok(subscriber_lists)
    }

    pub async fn update(&self, subscriber_id: i32, list_id: i32, dto: UpdateSubscriberListDto) -> Result<Option<SubscriberList>, ApiError> {
        let subscriber_list = sqlx::query_as!(
            SubscriberList,
            r#"
            UPDATE subscriber_lists
            SET 
                status = CASE 
                    WHEN $3::subscription_status IS NOT NULL 
                    THEN $3::subscription_status 
                    ELSE status 
                END,
                meta = CASE 
                    WHEN $4::jsonb IS NOT NULL 
                    THEN $4::jsonb 
                    ELSE meta 
                END,
                updated_at = NOW()
            WHERE subscriber_id = $1 AND list_id = $2
            RETURNING 
                subscriber_id,
                list_id,
                meta,
                status::subscription_status as "status: SubscriptionStatus",
                created_at,
                updated_at
            "#,
            subscriber_id,
            list_id,
            dto.status as Option<SubscriptionStatus>,
            dto.meta
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(subscriber_list)
    }

    pub async fn delete(&self, subscriber_id: i32, list_id: i32) -> Result<Option<()>, ApiError> {
        let result = sqlx::query!(
            r#"
            DELETE FROM subscriber_lists
            WHERE subscriber_id = $1 AND list_id = $2
            "#,
            subscriber_id,
            list_id
        )
        .execute(&self.pool)
        .await?;

        Ok((result.rows_affected() > 0).then_some(()))
    }
}
