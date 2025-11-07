use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct ServiceCampaign {
    pub id: Uuid,
    #[validate(length(min = 1, message = "Артикул не может быть пустым"))]
    pub article: String,
    #[validate(length(min = 1, message = "Название не может быть пустым"))]
    pub name: String,
    pub description: Option<String>,
    pub brand_id: Uuid,
    pub car_model_id: Uuid,
    pub target_vins: Vec<String>,
    pub required_parts: Vec<Uuid>,
    pub required_works: Vec<Uuid>,
    pub is_mandatory: bool,
    pub is_completed: bool,
    pub status: ServiceCampaignStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ServiceCampaignStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "cancelled")]
    Cancelled,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateServiceCampaignRequest {
    #[validate(length(min = 1))]
    pub article: String,
    #[validate(length(min = 1))]
    pub name: String,
    pub description: Option<String>,
    pub brand_id: Uuid,
    pub car_model_id: Uuid,
    pub target_vins: Vec<String>,
    pub required_parts: Vec<Uuid>,
    pub required_works: Vec<Uuid>,
    pub is_mandatory: bool,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateServiceCampaignRequest {
    pub article: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub brand_id: Option<Uuid>,
    pub car_model_id: Option<Uuid>,
    pub target_vins: Option<Vec<String>>,
    pub required_parts: Option<Vec<Uuid>>,
    pub required_works: Option<Vec<Uuid>>,
    pub is_mandatory: Option<bool>,
    pub is_completed: Option<bool>,
    pub status: Option<ServiceCampaignStatus>,
}