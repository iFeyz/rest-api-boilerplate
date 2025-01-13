use sqlx::PgPool;
use crate::{
    models::template::{Template, CreateTemplateDto, PaginationDto , TemplateType},
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
}