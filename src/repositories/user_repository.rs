use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::ApiError,
    models::user::{User , CreateUserDto , UpdateUserDto},
};

pub struct UserRepository {
    pool : PgPool,
}

impl UserRepository {
    pub fn new(pool : PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self , user : CreateUserDto) -> Result<User , ApiError> {
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (email, name)
            VALUES ($1, $2)
            RETURNING id, email, name, created_at as "created_at!", updated_at as "updated_at!"
            "#,
            user.email,
            user.name
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(user)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, ApiError> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT 
                id, 
                email, 
                name, 
                created_at as "created_at!", 
                updated_at as "updated_at!"
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn find_all(&self) -> Result<Vec<User>, ApiError> {
        let users = sqlx::query_as!(
            User,
            r#"
            SELECT 
                id, 
                email, 
                name, 
                created_at as "created_at!", 
                updated_at as "updated_at!"
            FROM users
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(users)
    }

    pub async fn update(&self, id: Uuid, user: UpdateUserDto) -> Result<User, ApiError> {
        let current_user = self.find_by_id(id).await?
            .ok_or_else(|| ApiError::NotFound)?;

        let user = sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET 
                email = $1,
                name = $2
            WHERE id = $3
            RETURNING id, email, name, created_at as "created_at!", updated_at as "updated_at!"
            "#,
            user.email.unwrap_or(current_user.email),
            user.name.unwrap_or(current_user.name),
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), ApiError> {
        let result = sqlx::query!(
            r#"
            DELETE FROM users
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(ApiError::NotFound);
        }

        Ok(())
    }
}