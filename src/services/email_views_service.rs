use crate::{
    models::email_views::{
        EmailView,
        CreateEmailViewDto,
        GetEmailViewDto,
        PaginationDto
    },
    repositories::email_views_repository::EmailViewsRepository,
    error::ApiError
};

pub struct EmailViewsService {
    repository: EmailViewsRepository
}

impl EmailViewsService {
    pub fn new(repository: EmailViewsRepository) -> Self {
        Self { repository }
    }

    pub async fn create_email_view(&self, dto: CreateEmailViewDto) -> Result<EmailView, ApiError> {
        println!("create_email_view: {:?}", dto);
        self.repository.create(dto).await
    }
}