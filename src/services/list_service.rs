use crate::{
    error::ApiError,
    models::list::{List, CreateListDto, ListPaginationDto},
    repositories::lists_repository::ListsRepository
};

pub struct ListService {
    repository: ListsRepository
}

impl ListService {
    pub fn new(repository: ListsRepository) -> Self {
        Self { repository }
    }

    pub async fn create_list(&self, dto: CreateListDto) -> Result<List, ApiError> {
        println!("Creating list: {:?}", dto);
        self.repository.create(dto).await
    }

    pub async fn get_list_by_id(&self, id: i32) -> Result<Option<List>, ApiError> {
        println!("Getting list by id: {}", id);
        self.repository.find_by_id(id).await
    }

    pub async fn get_lists(&self , pagination: ListPaginationDto) -> Result<Option<Vec<List>>, ApiError> {
        println!("Getting lists with pagination: {:?}", pagination);
        self.repository.find_all(&pagination).await
    }

    pub async fn delete_list(&self , id : i32) -> Result<Option<List>, ApiError> {
        println!("Deleting list by id: {}", id);
        self.repository.delete(id).await
    }
}