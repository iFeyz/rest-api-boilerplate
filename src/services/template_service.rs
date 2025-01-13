use crate::{
    models::template::{Template, CreateTemplateDto, UpdateTemplateDto, PaginationDto},
    repositories::template_repository::TemplateRepository,
    error::ApiError,
};

pub struct TemplateService {
    repository: TemplateRepository,
}

impl TemplateService {
    pub fn new(repository: TemplateRepository) -> Self {
        Self { repository }
    }

    pub async fn create_template(&self, template: CreateTemplateDto) -> Result<Template, ApiError> {
        self.repository.create(template).await
    }
}