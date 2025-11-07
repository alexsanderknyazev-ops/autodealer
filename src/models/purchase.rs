use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use validator::Validate;

use super::enums::RequestStatus;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PurchaseRequest {
    pub id: Uuid,
    pub car_id: Uuid,
    pub customer_id: Uuid,
    pub status: RequestStatus,
    pub offer_price: Option<f64>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreatePurchaseRequest {
    pub car_id: Uuid,
    pub customer_id: Uuid,
    #[validate(range(min = 0.0))]
    pub offer_price: Option<f64>,
    pub notes: Option<String>,
}