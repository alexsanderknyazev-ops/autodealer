use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Type)]
#[sqlx(type_name = "VARCHAR")]  // Указываем что храним как VARCHAR в БД
pub enum FuelType {
    #[sqlx(rename = "Petrol")]
    Petrol,
    #[sqlx(rename = "Diesel")]
    Diesel,
    #[sqlx(rename = "Electric")]
    Electric,
    #[sqlx(rename = "Hybrid")]
    Hybrid,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Type)]
#[sqlx(type_name = "VARCHAR")]
pub enum Transmission {
    #[sqlx(rename = "Manual")]
    Manual,
    #[sqlx(rename = "Automatic")]
    Automatic,
    #[sqlx(rename = "CVT")]
    CVT,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Type)]
#[sqlx(type_name = "VARCHAR")]
pub enum CarStatus {
    #[sqlx(rename = "Available")]
    Available,
    #[sqlx(rename = "Reserved")]
    Reserved,
    #[sqlx(rename = "Sold")]
    Sold,
    #[sqlx(rename = "Maintenance")]
    Maintenance,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Type)]
#[sqlx(type_name = "VARCHAR")]
pub enum RequestStatus {
    #[sqlx(rename = "Pending")]
    Pending,
    #[sqlx(rename = "Approved")]
    Approved,
    #[sqlx(rename = "Rejected")]
    Rejected,
    #[sqlx(rename = "Completed")]
    Completed,
}