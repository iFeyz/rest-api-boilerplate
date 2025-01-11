use sqlx::{PgPool, Execute};
use uuid::Uuid;

use crate::{
    error::ApiError,
    models::subscriber::{Subscriber, CreateSubscriberDto, SubscriberStatus, PaginationDto},
};

pub struct SubscriberRepository {
    pool: PgPool,
}

impl SubscriberRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, subscriber: CreateSubscriberDto) -> Result<Subscriber, ApiError> {
        let subscriber = sqlx::query_as!(
            Subscriber,
            r#"
            INSERT INTO subscribers (email, name, attribs, status)
            VALUES ($1, $2, $3, $4::subscriber_status)  -- Ajout du cast explicite
            RETURNING 
                id, uuid, email, name, 
                attribs,
                status as "status!: SubscriberStatus",
                created_at,
                updated_at
            "#,
            subscriber.email,
            subscriber.name,
            subscriber.attribs.unwrap_or_else(|| serde_json::json!({})),
            SubscriberStatus::Enabled as SubscriberStatus,  // Cast explicite
        )
        .fetch_one(&self.pool)
        .await?;
    
        Ok(subscriber)
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<Subscriber>, ApiError> {
        let subscriber = sqlx::query_as!(
            Subscriber,
            r#"
            SELECT 
                id, uuid, email, name,
                attribs,
                status as "status!: SubscriberStatus",
                created_at,
                updated_at
            FROM subscribers
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(subscriber)
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<Subscriber>, ApiError> {
        let subscriber = sqlx::query_as!(
            Subscriber,
            r#"
            SELECT 
                id, uuid, email, name, 
                attribs,
                status as "status!: SubscriberStatus",
                created_at, updated_at
            FROM subscribers 
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(subscriber)
    }

    pub async fn delete_by_id(&self, id: i32) -> Result<Option<Subscriber>, ApiError> {
        let subscriber = sqlx::query_as!(
            Subscriber,
            r#"
            DELETE FROM subscribers 
            WHERE id = $1 
            RETURNING 
                id, 
                uuid, 
                email, 
                name, 
                attribs,
                status as "status!: SubscriberStatus",
                created_at, 
                updated_at
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(subscriber)
    }

    pub async fn delete_by_email(&self, email: &str) -> Result<Option<Subscriber>, ApiError> {
        let subscriber = sqlx::query_as!(
            Subscriber,
            r#"
            DELETE FROM subscribers 
            WHERE email = $1 
            RETURNING 
                id, 
                uuid, 
                email, 
                name, 
                attribs,
                status as "status!: SubscriberStatus",
                created_at, 
                updated_at
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(subscriber)
    }

    pub async fn find_all(&self, params: &PaginationDto) -> Result<Option<Vec<Subscriber>>, ApiError> {
        let base_query = r#"
            SELECT 
                id, 
                uuid, 
                email, 
                name, 
                attribs,
                status::subscriber_status as "status",
                created_at, 
                updated_at
            FROM subscribers"#;

        let mut query = sqlx::QueryBuilder::new(base_query);

        // Start WHERE clause if needed
        let mut first_condition = true;

        // Search query
        if let Some(search) = &params.query {
            if first_condition {
                query.push(" WHERE ");
                first_condition = false;
            }
            query.push(" (email ILIKE ");
            let pattern = format!("%{}%", search);
            query.push_bind(pattern.clone());
            query.push(" OR name ILIKE ");
            query.push_bind(pattern);
            query.push(")");
        }

        // List ID filter
        if let Some(list_ids) = &params.list_id {
            if first_condition {
                query.push(" WHERE ");
                first_condition = false;
            } else {
                query.push(" AND ");
            }
            query.push(" id = ANY(");
            query.push_bind(list_ids);
            query.push(")");
        }

        // Status filter
        if let Some(status) = &params.subscription_status {
            if first_condition {
                query.push(" WHERE ");
            } else {
                query.push(" AND ");
            }
            query.push(" status = ");
            query.push_bind(status);
        }

        // Ordering
        let order_by = match params.order_by.as_str() {
            "name" | "status" | "created_at" | "updated_at" => &params.order_by,
            _ => "created_at"
        };
        let order = match params.order.to_uppercase().as_str() {
            "ASC" | "DESC" => params.order.to_uppercase(),
            _ => "DESC".to_string()
        };
        query.push(&format!(" ORDER BY {} {}", order_by, order));

        // Pagination
        if params.per_page > 0 {
            let offset = (params.page - 1) * params.per_page;
            query.push(&format!(" LIMIT {} OFFSET {}", params.per_page, offset));
        }

        let built_query = query.build_query_as::<Subscriber>();
        println!("Executing query: {}", built_query.sql());

        let subscribers = built_query.fetch_all(&self.pool).await?;

        if subscribers.is_empty() {
            Ok(None)
        } else {
            Ok(Some(subscribers))
        }
    }

    pub async fn update_by_id(&self, id: i32, subscriber: Subscriber) -> Result<Option<Subscriber>, ApiError> {
        let subscriber = sqlx::query_as!(
            Subscriber,
            r#"
            UPDATE subscribers SET 
                name = $2, 
                attribs = $3, 
                status = $4::subscriber_status 
            WHERE id = $1 
            RETURNING 
                id, 
                uuid, 
                email, 
                name, 
                attribs,
                status as "status!: SubscriberStatus",
                created_at, 
                updated_at
            "#,
            id,
            subscriber.name,
            subscriber.attribs,
            subscriber.status as SubscriberStatus
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(subscriber)
    }

    pub async fn update_by_email(&self, email: &str, subscriber: Subscriber) -> Result<Option<Subscriber>, ApiError> {
        let subscriber = sqlx::query_as!(
            Subscriber,
            r#"
            UPDATE subscribers SET 
                name = $2, 
                attribs = $3, 
                status = $4::subscriber_status 
            WHERE email = $1 
            RETURNING 
                id, 
                uuid, 
                email, 
                name, 
                attribs,
                status as "status!: SubscriberStatus",
                created_at, 
                updated_at
            "#,
            email,
            subscriber.name,
            subscriber.attribs,
            subscriber.status as SubscriberStatus
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(subscriber)
    }
}