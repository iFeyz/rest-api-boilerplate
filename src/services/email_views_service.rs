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
use actix_web::{HttpRequest, HttpResponse};
use std::net::IpAddr;
use maxminddb::geoip2;
use std::str::FromStr;
use tracing::{info, warn, error};

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
        geoip_reader: &maxminddb::Reader<Vec<u8>>
    ) -> Result<EmailView, ApiError> {
        fn get_geo_location(
            subscriber_id: i32,
            sequence_email_id: i32,
            campaign_id: i32,
            ip_address: String,
            reader: &maxminddb::Reader<Vec<u8>>,
            user_agent: &str
        ) -> Option<CreateEmailViewDto> {
            let ip = IpAddr::from_str(ip_address.as_str()).ok()?;
            let city_info: geoip2::City = reader.lookup(ip).ok()?;
    
            Some(CreateEmailViewDto {
                sequence_email_id,
                subscriber_id,
                campaign_id,
                ip_address: Some(ip_address),
                user_agent: Some(user_agent.to_string()),
                country: Some(city_info
                    .country
                    .and_then(|country| country.names)
                    .and_then(|names| names.get("en").cloned())
                    .map(|name| name.to_string())
                    .unwrap_or_else(|| "Unknown".to_string())),
                city: Some(city_info
                    .city
                    .and_then(|city| city.names)
                    .and_then(|names| names.get("en").cloned())
                    .map(|name| name.to_string())
                    .unwrap_or_else(|| "Unknown".to_string())),
                region: Some(city_info
                    .subdivisions
                    .and_then(|subdivisions| subdivisions.get(0).cloned())
                    .and_then(|subdivision| subdivision.names)
                    .and_then(|names| names.get("en").cloned())
                    .map(|name| name.to_string())
                    .unwrap_or_else(|| "Unknown".to_string())),
                latitude: Some(city_info
                    .location.as_ref()
                    .and_then(|location| location.latitude)
                    .unwrap_or(0.0)
                    .to_string()),
                longitude: Some(city_info
                    .location.as_ref()
                    .and_then(|location| location.longitude)
                    .unwrap_or(0.0)
                    .to_string()),
                metadata: None
            })
        }

        let (subscriber_id, sequence_email_id, campaign_id) = path;
        
        // Gestion des erreurs pour User-Agent
        let user_agent = req.headers()
            .get("User-Agent")
            .and_then(|h| h.to_str().ok())
            .unwrap_or("Unknown");

        let ip_address = req.peer_addr()
            .map(|addr| addr.ip().to_string())
            .unwrap_or_else(|| "Unknown".to_string());

        let geo_location = get_geo_location(
            subscriber_id,
            sequence_email_id,
            campaign_id,
            ip_address.clone(),
            geoip_reader,
            user_agent
        );

        match geo_location {
            Some(location_dto) => {
                info!("Creating email view with geolocation: {:?}", &location_dto);
                self.repository.create(location_dto).await
            }
            None => {
                // Fallback sans géolocalisation
                warn!("Could not get geolocation, creating email view with basic info");
                let basic_dto = CreateEmailViewDto {
                    sequence_email_id,
                    subscriber_id,
                    campaign_id,
                    ip_address: Some(ip_address),
                    user_agent: Some(user_agent.to_string()),
                    country: None,
                    city: None,
                    region: None,
                    latitude: None,
                    longitude: None,
                    metadata: None,
                };
                self.repository.create(basic_dto).await
            }
        }
    }
}