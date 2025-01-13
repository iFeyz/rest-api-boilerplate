use sqlx::PgPool;
use crate::{
    models::template::{Template, CreateTemplateDto, PaginationDto, TemplateType, UpdateTemplateDto},
    error::ApiError,
};

pub struct TemplateRepository {
    pool: PgPool,
}

impl TemplateRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, template: CreateTemplateDto) -> Result<Template, ApiError> {
        let template = sqlx::query_as!(
            Template,
            r#"
            INSERT INTO templates (name, type, subject, body, is_default)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, name, type as "template_type: TemplateType", subject, body, is_default, 
                created_at "created_at!", updated_at "updated_at!"
            "#,
            template.name, template.template_type as TemplateType, template.subject, template.body, template.is_default
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(template)
    }

    pub async fn find_all(&self, pagination: &PaginationDto) -> Result<Vec<Template>, ApiError> {
        let mut query = String::from(
            r#"
            SELECT id, name, type::template_type as "type", subject, body, is_default, 
                created_at "created_at", updated_at "updated_at"
            FROM templates
            "#,
        );

        let mut conditions = Vec::new();
        let mut params = Vec::new();
        let mut param_count = 1;

        if let Some(search) = &pagination.query {
            conditions.push(format!("(name ILIKE ${} OR subject ILIKE ${} OR body ILIKE ${})",
                param_count, param_count + 1, param_count + 2));
            let search_pattern = format!("%{}%", search);
            params.push(search_pattern.clone());
            params.push(search_pattern.clone());
            params.push(search_pattern);
            param_count += 3;
        }

        if !conditions.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&conditions.join(" AND "));
        }

        query.push_str(&format!(" ORDER BY {} {}", pagination.order_by, pagination.order));
        query.push_str(&format!(" LIMIT {} OFFSET {}", 
            pagination.per_page, 
            (pagination.page - 1) * pagination.per_page
        ));

        let templates = sqlx::query_as::<_, Template>(&query)
            .fetch_all(&self.pool)
            .await?;

        Ok(templates)
    }

    pub async fn update(&self, id: i32, template: UpdateTemplateDto) -> Result<Template, ApiError> {
        let current = sqlx::query_as!(
            Template,
            r#"
            SELECT id, name, type as "template_type: TemplateType", subject, body, is_default, 
                created_at "created_at!", updated_at "updated_at!"
            FROM templates 
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or(ApiError::NotFound)?;

        let template = sqlx::query_as!(
            Template,
            r#"
            UPDATE templates 
            SET 
                name = COALESCE($1, name),
                subject = COALESCE($2, subject),
                body = COALESCE($3, body),
                is_default = COALESCE($4, is_default),
                updated_at = NOW()
            WHERE id = $5
            RETURNING id, name, type as "template_type: TemplateType", subject, body, is_default, 
                created_at "created_at!", updated_at "updated_at!"
            "#,
            template.name,
            template.subject,
            template.body,
            template.is_default,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(template)
    }

    pub async fn delete(&self, id: i32) -> Result<(), ApiError> {
        let result = sqlx::query!(
            "DELETE FROM templates WHERE id = $1",
            id
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(ApiError::NotFound);
        }

        Ok(())
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Template, ApiError> {
        let template = sqlx::query_as!(
            Template,
            r#"
            SELECT id, name, type as "template_type: TemplateType", subject, body, is_default, 
                created_at "created_at!", updated_at "updated_at!"
            FROM templates 
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or(ApiError::NotFound)?;

        Ok(template)
    }
}