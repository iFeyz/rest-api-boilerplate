use actix_web::{test, web, App};
use sqlx::PgPool;
use crate::{
    api::template::create_template,
    models::template::{CreateTemplateDto, TemplateType},
    repositories::template_repository::TemplateRepository,
    services::template_service::TemplateService,
    middleware::auth::AuthMiddleware,
};

async fn setup() -> (PgPool, String) {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let api_key = std::env::var("API_KEY").expect("API_KEY must be set");
    
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    sqlx::query!("TRUNCATE TABLE templates RESTART IDENTITY CASCADE")
        .execute(&pool)
        .await
        .expect("Failed to truncate templates table");

    (pool, api_key)
}

#[actix_web::test]
async fn test_create_template() {
    let (pool, api_key) = setup().await;
    
    let template_repository = TemplateRepository::new(pool.clone());
    let template_service = web::Data::new(TemplateService::new(template_repository));

    let app = test::init_service(
        App::new()
            .wrap(AuthMiddleware::new(api_key.clone()))
            .app_data(template_service.clone())
            .service(create_template)
    ).await;

    let template_data = CreateTemplateDto {
        name: "Test Template".to_string(),
        template_type: TemplateType::Campaign,
        subject: "Test Subject".to_string(),
        body: "Test Body".to_string(),
        is_default: false,
    };

    let req = test::TestRequest::post()
        .uri("/api/templates")
        .set_json(&template_data)
        .insert_header(("x-api-key", api_key))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let created_template: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(created_template["name"], "Test Template");
    assert_eq!(created_template["template_type"], "campaign");
    assert_eq!(created_template["subject"], "Test Subject");
    assert_eq!(created_template["body"], "Test Body");
    assert_eq!(created_template["is_default"], false);
} 