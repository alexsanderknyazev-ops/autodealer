use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct Part {
    pub id: Uuid,
    #[validate(length(min = 1, message = "Артикул не может быть пустым"))]
    pub article: String,
    #[validate(length(min = 1, message = "Название не может быть пустым"))]
    pub name: String,
    pub brand_id: Uuid, // ← ДОБАВЛЯЕМ ссылку на бренд
    pub car_model_id: Uuid, // ← ДОБАВЛЯЕМ ссылку на модель
    #[validate(range(min = 0.0))]
    pub purchase_price: f64,
    #[validate(range(min = 0.0))]
    pub sale_price: f64,
    pub compatible_vins: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreatePartRequest {
    #[validate(length(min = 1))]
    pub article: String,
    #[validate(length(min = 1))]
    pub name: String,
    pub brand_id: Uuid, // ← ДОБАВЛЯЕМ
    pub car_model_id: Uuid, // ← ДОБАВЛЯЕМ
    #[validate(range(min = 0.0))]
    pub purchase_price: f64,
    #[validate(range(min = 0.0))]
    pub sale_price: f64,
    pub compatible_vins: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdatePartRequest {
    pub article: Option<String>,
    pub name: Option<String>,
    pub brand_id: Option<Uuid>,
    pub car_model_id: Option<Uuid>,
    #[validate(range(min = 0.0))]
    pub purchase_price: Option<f64>,
    #[validate(range(min = 0.0))]
    pub sale_price: Option<f64>,
    pub compatible_vins: Option<Vec<String>>,
}