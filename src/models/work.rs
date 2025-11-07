use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct Work {
    pub id: Uuid,
    #[validate(length(min = 1, message = "Наименование не может быть пустым"))]
    pub name: String,
    #[validate(length(min = 1, message = "Артикул не может быть пустым"))]
    pub article: String,
    #[validate(range(min = 0.1, message = "Норма часов должна быть больше 0"))]
    pub norm_hours: f64,
    pub brand_id: Uuid,
    pub car_model_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateWorkRequest {
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(length(min = 1))]
    pub article: String,
    #[validate(range(min = 0.1))]
    pub norm_hours: f64,
    pub brand_id: Uuid,
    pub car_model_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateWorkRequest {
    pub name: Option<String>,
    pub article: Option<String>,
    #[validate(range(min = 0.1))]
    pub norm_hours: Option<f64>,
    pub brand_id: Option<Uuid>,
    pub car_model_id: Option<Uuid>,
}