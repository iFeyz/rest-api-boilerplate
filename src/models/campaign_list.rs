use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CampaignList {
    pub id: i32,
    pub campaign_id: i32,
    pub list_id: i32,
    pub list_name: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateCampaignListDto {
    pub campaign_id: i32,
    pub list_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCampaignListDto {
    pub list_id: i32,
    pub list_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GetCampaignListDto {
    pub campaign_id: i32,
    pub list_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct DeleteCampaignListDto {
    pub campaign_id: i32,
    pub list_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct PaginationDto {
    pub campaign_id: Option<i32>,
    pub list_id: Option<i32>,

    #[serde(rename = "order_by")]
    #[serde(default = "default_order_by")]
    pub order_by: String,
    
    #[serde(default = "default_order")]
    pub order: String,
    
    #[serde(default = "default_page")]
    pub page: i32,
    
    #[serde(rename = "per_page")]
    #[serde(default = "default_per_page")]
    pub per_page: i32,
}

fn default_order_by() -> String {
    "created_at".to_string()
}

fn default_order() -> String {
    "DESC".to_string()
}

fn default_page() -> i32 {
    1
}

fn default_per_page() -> i32 {
    10
}
