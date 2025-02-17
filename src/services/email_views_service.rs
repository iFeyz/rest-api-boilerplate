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
use actix_web::HttpRequest;
use maxminddb::{geoip2::City, Reader};
use tracing::info;
use serde_json;

pub struct EmailViewsService {
    repository: EmailViewsRepository
}

impl EmailViewsService {
    pub fn new(repository: EmailViewsRepository) -> Self {
        Self { repository }
    }

    pub async fn create_email_view(&self, dto: CreateEmailViewDto) -> Result<EmailView, ApiError> {
        info!("Creating email view: {:?}", dto);
        self.repository.create(dto).await
    }

    pub async fn get_email_view(
        &self,
        req: HttpRequest,
        path: (i32, i32, i32),
        geoip_reader: &Reader<Vec<u8>>,
    ) -> Result<EmailView, ApiError> {
        let (subscriber_id, sequence_email_id, campaign_id) = path;
        
        // Get IP address from request
        let ip_address = req
            .connection_info()
            .realip_remote_addr()
            .map(|s| s.to_string());

        // Get user agent from request
        let user_agent = req
            .headers()
            .get("User-Agent")
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string());

        tracing::info!(
            "Processing email view - IP: {:?}, UA: {:?}",
            ip_address,
            user_agent
        );

        let mut location_info: Option<City> = None;
        if let Some(ip) = ip_address.as_ref().and_then(|ip| ip.parse().ok()) {
            if let Ok(city) = geoip_reader.lookup(ip) {
                location_info = Some(city);
                tracing::info!("Found location info: {:?}", location_info);
            }
        }

        let dto = CreateEmailViewDto {
            subscriber_id,
            sequence_email_id,
            campaign_id,
            ip_address,
            user_agent,
            country: location_info.as_ref()
                .and_then(|l| l.country.as_ref())
                .and_then(|c| c.names.as_ref())
                .and_then(|n| n.get("en"))
                .map(|s| s.to_string()),
            city: location_info.as_ref()
                .and_then(|l| l.city.as_ref())
                .and_then(|c| c.names.as_ref())
                .and_then(|n| n.get("en"))
                .map(|s| s.to_string()),
            region: location_info.as_ref()
                .and_then(|l| l.subdivisions.as_ref())
                .and_then(|s| s.first())
                .and_then(|s| s.names.as_ref())
                .and_then(|n| n.get("en"))
                .map(|s| s.to_string()),
            latitude: location_info.as_ref()
                .and_then(|l| l.location.as_ref())
                .and_then(|l| l.latitude)
                .map(|lat| lat.to_string()),
            longitude: location_info.as_ref()
                .and_then(|l| l.location.as_ref())
                .and_then(|l| l.longitude)
                .map(|lon| lon.to_string()),
            metadata: Some(serde_json::json!({})),
        };

        self.create_email_view(dto).await
    }
}