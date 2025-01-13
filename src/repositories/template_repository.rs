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
            SELECT id, name,  type::template_type as "type", subject, body, is_default, 
                created_at "created_at", updated_at "updated_at"
            FROM templates
            "#,
        );

        let mut conditions = Vec::new();
        let mut params: Vec<String> = Vec::new();

        if let Some(search) = &pagination.query {
            if !search.is_empty() {
                conditions.push("(name ILIKE $1 OR subject ILIKE $1 OR body ILIKE $1)");
                params.push(format!("%{}%", search));
            }
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

        let query = sqlx::query_as::<_, Template>(&query);

        let query = if !params.is_empty() {
            query.bind(&params[0])
        } else {
            query
        };

        let templates = query.fetch_all(&self.pool).await?;

        Ok(templates)
    }

    pub async fn update(&self, id: i32, template: UpdateTemplateDto) -> Result<Option<Template>, ApiError> {
        let template = sqlx::query_as!(
            Template,
            r#"
            UPDATE templates SET 
                name = COALESCE($2, name),
                type = COALESCE($3, type),
                subject = COALESCE($4, subject),
                body = COALESCE($5, body),
                is_default = COALESCE($6, is_default),
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, name, type as "template_type: TemplateType", subject, body, is_default, 
                created_at "created_at!", updated_at "updated_at!"
            "#,
            id,
            template.name,
            template.template_type as Option<TemplateType>,
            template.subject,
            template.body,
            template.is_default
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(template)
    }

    pub async fn delete(&self, id: i32) -> Result<Option<Template>, ApiError> {
        let template = sqlx::query_as!(
            Template,
            r#"
            DELETE FROM templates WHERE id = $1
            RETURNING id, name, type as "template_type: TemplateType", subject, body, is_default, 
                created_at "created_at", updated_at "updated_at"
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(template)
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