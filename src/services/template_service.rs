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

    pub async fn create(&self, template: CreateTemplateDto) -> Result<Template, ApiError> {
        self.repository.create(template).await
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Template, ApiError> {
        self.repository.find_by_id(id).await
    }

    pub async fn find_all(&self, pagination: PaginationDto) -> Result<Vec<Template>, ApiError> {
        self.repository.find_all(&pagination).await
    }

    pub async fn update(&self, id: i32, template: UpdateTemplateDto) -> Result<Option<Template>, ApiError> {
        self.repository.update(id, template).await
    }

    pub async fn delete(&self, id: i32) -> Result<Option<Template>, ApiError> {
        self.repository.delete(id).await
    }
}