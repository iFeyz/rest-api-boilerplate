use sqlx::{PgPool, Execute, QueryBuilder};
use sqlx::postgres::Postgres;
use uuid::Uuid;

use crate::{
    error::ApiError,
    models::subscriber::{Subscriber,  CreateSubscriberDto, SubscriberStatus, PaginationParams, SubscriberFilter, SubscriberResponse},
};
#[derive(Clone)]
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
            VALUES ($1, $2, $3, $4::subscriber_status) 
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

    pub async fn find_all(
        &self,
        filter: Option<SubscriberFilter>,
        pagination: Option<PaginationParams>,
    ) -> Result<SubscriberResponse<Subscriber>, ApiError> {
        let pagination = pagination.unwrap_or_default();
        let offset = (pagination.page.unwrap_or_else(|| 1) - 1) * pagination.per_page.unwrap_or_else(|| 10);
    
        // Préparer la liste des conditions et des paramètres dynamiques
        let mut conditions = Vec::new();
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
            if let Some(email) = filter.email {
                conditions.push(format!("email = {}", email));
                params.push(email);
            }
            if let Some(name) = filter.name {
                conditions.push(format!("name = {}", name));
                params.push(name);
            }
            if let Some(status) = filter.status {
                conditions.push(format!("status = {}", status));
                params.push(status.to_string());
            }
        }
    
        // Créer la clause WHERE uniquement si des conditions sont définies
        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };
    
        // Construire la requête SQL
        let query = format!(
            "SELECT * FROM subscribers {} ORDER BY {} {} LIMIT {} OFFSET {}",
            where_clause,
            pagination.sort_by.unwrap_or_else(|| "id".to_string()),
            pagination.sort_order.unwrap_or_else(|| "ASC".to_string()),
            pagination.per_page.unwrap_or_else(|| 10),
            offset
        );
    
        // Construire la requête avec les paramètres
        let mut query_builder = sqlx::query_as::<_, Subscriber>(&query);
    
        for param in params {
            query_builder = query_builder.bind(param);
        }
    
        // Ajouter les paramètres de pagination
        query_builder = query_builder
            .bind(pagination.per_page.unwrap_or_else(|| 10) as i64)
            .bind(offset as i64);
    
        println!("{}", query_builder.sql());
        // Exécuter la requête
        let subscribers = query_builder
            .fetch_all(&self.pool)
            .await?;
    
        // Retourner les résultats
        Ok(SubscriberResponse {
            items: subscribers,
            page: pagination.page.unwrap_or_else(|| 1),
            per_page: pagination.per_page.unwrap_or_else(|| 10),
        })
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