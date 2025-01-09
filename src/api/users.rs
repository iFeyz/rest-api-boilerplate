use actix_web::{web, HttpResponse, post, get};
use uuid::Uuid;
use crate::{
    models::user::{CreateUserDto, GetUserDto},
    services::user_service::UserService,
    error::ApiError,
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .service(create_user)
            .service(get_user)
    );
}

#[post("/create")]
async fn create_user(
    service: web::Data<UserService>,
    user: web::Json<CreateUserDto>
) -> Result<HttpResponse, ApiError> {
    let user = service.create_user(user.into_inner()).await?;
    Ok(HttpResponse::Created().json(user))
}
// Query with id in the url
#[get("")]
async fn get_user(
    service: web::Data<UserService>,
    query: web::Query<GetUserDto>
) -> Result<HttpResponse, ApiError> {
    let user = service.get_user(query.into_inner()).await?;
    match user {
        Some(user) => Ok(HttpResponse::Ok().json(user)),
        None => Err(ApiError::NotFound)
    }
}