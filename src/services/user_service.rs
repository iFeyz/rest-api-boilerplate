use uuid::Uuid;
use crate::{
    error::ApiError,
    models::user::{User , CreateUserDto , UpdateUserDto , GetUserDto},
    repositories::user_repository::UserRepository,
};

pub struct UserService {
    repository : UserRepository,
}

impl UserService {
    pub fn new(repository : UserRepository) -> Self {
        Self { repository }
    }

    pub async fn create_user(&self , dto : CreateUserDto) -> Result<User , ApiError> {
        println!("Creating user: {:?}", dto);
        self.repository.create(dto).await
    }

    pub async fn get_user(&self , dto : GetUserDto) -> Result<Option<User> , ApiError> {
        println!("Getting user: {:?}", dto.id);
        self.repository.find_by_id(dto.id).await
    }


}