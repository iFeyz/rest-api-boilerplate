use sqlx::PgPool;
use crate::{
    error::ApiError,
    models::list::{List, CreateListDto, ListPaginationDto, UpdateListDto, ListType, ListOptin}
};
use crate::monitoring::DatabaseMetrics;

pub struct ListsRepository {
    pool: PgPool,
    db_metrics: DatabaseMetrics,
}

impl ListsRepository {
    pub fn new(pool: PgPool, db_metrics: DatabaseMetrics) -> Self {
        Self { pool, db_metrics }
    }

    pub async fn create(&self, list: CreateListDto) -> Result<List, ApiError> {
        let list = sqlx::query_as!(
            List,
            r#"
            INSERT INTO lists (name, "type", optin, tags, description)
            VALUES ($1, $2, $3, $4::text[], $5)
            RETURNING 
                id, 
                uuid, 
                name, 
                "type" as "type: _",
                optin as "optin: _",
                tags as "tags!: Vec<String>",
                description,
                created_at,
                updated_at
            "#,
            list.name,
            list.r#type as _,
            list.optin as _,
            &list.tags,
            list.description
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(list)
    }

    pub async fn find_by_id(&self , id : i32) -> Result<Option<List>, ApiError> {
        let list = sqlx::query_as!(
            List,
            r#"
            SELECT
                id , uuid , name , "type" as "type!: ListType",
                optin as "optin!: ListOptin",
                tags as "tags!: Vec<String>",
                description,
                created_at,
                updated_at
            FROM lists
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(list)
    }

    pub async fn find_all(&self , pagination : &ListPaginationDto) -> Result<Option<Vec<List>>, ApiError> {
        let db_metrics = self.db_metrics.clone();
        
        db_metrics.measure_query("select", "lists", async {
            let base_query = r#"
                SELECT
                    id , uuid , name ,
                    type::list_type as "type",
                    optin::list_optin as "optin",
                    tags as "tags!: Vec<String>",
                    description,
                    created_at,
                    updated_at
                FROM lists
            "#;
            
            let mut query = sqlx::QueryBuilder::new(base_query);

            // Start WHERE clause if needed
            let mut first_condition = true;

            //Search Query
            if let Some(search) = &pagination.query {
                if first_condition {
                    query.push(" WHERE ");
                    first_condition = false;
                }
                query.push(" (name ILIKE ");
                let pattern = format!("%{}%", search);
                query.push_bind(pattern.clone());
                query.push(")");
            }


            //Tags filter
            if let Some(tags) = &pagination.tags {
                if first_condition {
                    query.push(" WHERE ");
                    first_condition = false;
                } else {
                    query.push(" AND ");
                }
                query.push(" tags = ");
                query.push_bind(tags);
                
            }

            //Ordering
            let order_by = match pagination.order_by.as_str() {
                "name" | "type" | "optin" | "created_at" | "updated_at" => &pagination.order_by,
                _ => "created_at"
            };
            let order = match pagination.order.to_uppercase().as_str() {
                "ASC" | "DESC" => pagination.order.to_uppercase(),
                _ => "DESC".to_string()
            };
            query.push(&format!(" ORDER BY {} {}", order_by, order));

            //Pagination
            if pagination.per_page > 0 {
                let offset = (pagination.page - 1) * pagination.per_page;
                query.push(&format!(" LIMIT {} OFFSET {}", pagination.per_page, offset));
            }

            let built_query = query.build_query_as::<List>();
            //println!("Executing query: {}", built_query.sql());

            let lists = built_query.fetch_all(&self.pool).await?;

            if lists.is_empty() {
                Ok(None)
            } else {
                Ok(Some(lists))
            }
        }).await
    }

    pub async fn delete(&self , id : i32) -> Result<Option<List>, ApiError> {
        let list = sqlx::query_as!(
            List,
            r#"
            DELETE FROM lists WHERE id = $1
            RETURNING 
                id, 
                uuid, 
                name, 
                "type" as "type!: ListType",
                optin as "optin!: ListOptin",
                tags as "tags!: Vec<String>",
                description,
                created_at,
                updated_at
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(list)
    }

    pub async fn update(&self, list: UpdateListDto) -> Result<List, ApiError> {
        let list = sqlx::query_as!(
            List,
            r#"
            UPDATE lists 
            SET 
                name = CASE WHEN $1::text IS NOT NULL THEN $1 ELSE name END,
                "type" = CASE WHEN $2::text IS NOT NULL THEN $2::list_type ELSE "type" END,
                optin = CASE WHEN $3::text IS NOT NULL THEN $3::list_optin ELSE optin END,
                tags = CASE WHEN $4::text[] IS NOT NULL THEN $4 ELSE tags END,
                description = CASE WHEN $5::text IS NOT NULL THEN $5 ELSE description END
            WHERE id = $6
            RETURNING 
                id, 
                uuid,
                name,
                "type" as "type!: ListType",
                optin as "optin!: ListOptin",
                tags as "tags!: Vec<String>",
                description,
                created_at,
                updated_at
            "#,
            list.name.as_deref(),
            list.r#type.map(|t| t.to_string()),
            list.optin.map(|o| o.to_string()),
            list.tags.as_deref(),
            list.description,
            list.id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(list)
    }
}