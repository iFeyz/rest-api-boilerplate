use sqlx::PgPool;
use crate::{
    models::template::{Template, CreateTemplateDto, UpdateTemplateDto, GetTemplateDto, PaginationDto},
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
            INSERT INTO templates (name, subject, body, is_default)
            VALUES ($1, $2, $3, $4)
            RETURNING id, name, subject, body, is_default, created_at, updated_at
            "#,
            template.name, template.subject, template.body, template.is_default
        )
    }
}