use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use validator::Validate;

use super::enums::{FuelType, Transmission, CarStatus};

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct Car {
    pub id: Uuid,
    pub brand_id: Uuid,
    pub model_id: Uuid,
    pub year: i32,
    #[validate(range(min = 0.0))]
    pub price: f64,
    pub mileage: i32,
    pub color: String,
    #[validate(length(min = 17, max = 17, message = "VIN код должен содержать 17 символов"))]
    pub vin: String,
    pub fuel_type: FuelType,
    pub transmission: Transmission,
    pub status: CarStatus,
    pub completed_service_campaigns: Vec<Uuid>, // ← ДОБАВЛЯЕМ
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateCarRequest {
    pub brand_id: Uuid,
    pub model_id: Uuid,
    #[validate(range(min = 1990, max = 2024))]
    pub year: i32,
    #[validate(range(min = 0.0))]
    pub price: f64,
    pub mileage: i32,
    pub color: String,
    #[validate(length(min = 17, max = 17))]
    pub vin: String,
    pub fuel_type: FuelType,
    pub transmission: Transmission,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateCarRequest {
    pub brand_id: Option<Uuid>,
    pub model_id: Option<Uuid>,
    #[validate(range(min = 1990, max = 2024))]
    pub year: Option<i32>,
    #[validate(range(min = 0.0))]
    pub price: Option<f64>,
    pub mileage: Option<i32>,
    pub color: Option<String>,
    #[validate(length(min = 17, max = 17))]
    pub vin: Option<String>,
    pub fuel_type: Option<FuelType>,
    pub transmission: Option<Transmission>,
    pub status: Option<CarStatus>,
    pub completed_service_campaigns: Option<Vec<Uuid>>, // ← ДОБАВЛЯЕМ
}