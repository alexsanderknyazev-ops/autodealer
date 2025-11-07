use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct Brand {
    pub id: Uuid,
    #[validate(length(min = 1, message = "Название бренда не может быть пустым"))]
    pub name: String,
    pub country: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateBrandRequest {
    #[validate(length(min = 1))]
    pub name: String,
    pub country: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateBrandRequest {
    pub name: Option<String>,
    pub country: Option<String>,
}