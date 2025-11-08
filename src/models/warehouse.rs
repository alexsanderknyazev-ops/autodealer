use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct WarehouseItem {
    pub id: Uuid,
    pub part_id: Uuid,
    #[validate(range(min = 0, message = "Количество не может быть отрицательным"))]
    pub quantity: i32,
    #[validate(range(min = 0, message = "Минимальный запас не может быть отрицательным"))]
    pub min_stock_level: i32,
    #[validate(range(min = 0, message = "Максимальный запас не может быть отрицательным"))]
    pub max_stock_level: i32,
    pub location: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WarehouseItemWithPart {
    pub id: Uuid,
    pub part_id: Uuid,
    pub part_article: String,
    pub part_name: String,
    pub quantity: i32,
    pub min_stock_level: i32,
    pub max_stock_level: i32,
    pub location: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateWarehouseItemRequest {
    pub part_id: Uuid,
    #[validate(range(min = 0))]
    pub quantity: i32,
    #[validate(range(min = 0))]
    pub min_stock_level: Option<i32>,
    #[validate(range(min = 0))]
    pub max_stock_level: Option<i32>,
    pub location: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateWarehouseItemRequest {
    #[validate(range(min = 0))]
    pub quantity: Option<i32>,
    #[validate(range(min = 0))]
    pub min_stock_level: Option<i32>,
    #[validate(range(min = 0))]
    pub max_stock_level: Option<i32>,
    pub location: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct StockMovementRequest {
    #[validate(range(min = 1, message = "Количество должно быть положительным"))]
    pub quantity: i32,
    pub movement_type: StockMovementType,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum StockMovementType {
    #[serde(rename = "incoming")]
    Incoming,
    #[serde(rename = "outgoing")]
    Outgoing,
    #[serde(rename = "adjustment")]
    Adjustment,
}