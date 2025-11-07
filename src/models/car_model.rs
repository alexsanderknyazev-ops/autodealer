use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct CarModel {
    pub id: Uuid,
    #[validate(length(min = 1, message = "Название модели не может быть пустым"))]
    pub name: String,
    pub brand_id: Uuid, // Ссылка на бренд
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateCarModelRequest {
    #[validate(length(min = 1))]
    pub name: String,
    pub brand_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateCarModelRequest {
    pub name: Option<String>,
    pub brand_id: Option<Uuid>,
}