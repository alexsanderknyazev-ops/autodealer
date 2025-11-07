use async_trait::async_trait;
use sqlx::Error;
use uuid::Uuid;

use crate::models::{Car, CreateCarRequest, UpdateCarRequest, CarStatus};
use crate::database::DbPool;

#[async_trait]
pub trait CarRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Car>, Error>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Car>, Error>;
    async fn find_by_status(&self, status: CarStatus) -> Result<Vec<Car>, Error>;
    async fn find_by_brand_id(&self, brand_id: Uuid) -> Result<Vec<Car>, Error>;
    async fn find_by_model_id(&self, model_id: Uuid) -> Result<Vec<Car>, Error>;
    async fn find_by_vin(&self, vin: &str) -> Result<Option<Car>, Error>;
    async fn exists_by_vin(&self, vin: &str) -> Result<bool, Error>;
    async fn save(&self, create_request: &CreateCarRequest) -> Result<Car, Error>;
    async fn update(&self, id: Uuid, update_request: &UpdateCarRequest) -> Result<Option<Car>, Error>;
    async fn delete(&self, id: Uuid) -> Result<bool, Error>;
    async fn update_status(&self, id: Uuid, status: CarStatus) -> Result<Option<Car>, Error>;
}

#[derive(Clone)]
pub struct CarRepositoryImpl {
    pool: DbPool,
}

impl CarRepositoryImpl {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CarRepository for CarRepositoryImpl {
    async fn find_all(&self) -> Result<Vec<Car>, Error> {
        sqlx::query_as!(
            Car,
            r#"
            SELECT id, brand_id, model_id, year, price, mileage, color, vin,
                   fuel_type as "fuel_type: _", transmission as "transmission: _",
                   status as "status: _", created_at, updated_at
            FROM cars
            ORDER BY created_at DESC
            "#
        )
            .fetch_all(&self.pool)
            .await
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Car>, Error> {
        sqlx::query_as!(
            Car,
            r#"
            SELECT id, brand_id, model_id, year, price, mileage, color, vin,
                   fuel_type as "fuel_type: _", transmission as "transmission: _",
                   status as "status: _", created_at, updated_at
            FROM cars
            WHERE id = $1
            "#,
            id
        )
            .fetch_optional(&self.pool)
            .await
    }

    async fn find_by_status(&self, status: CarStatus) -> Result<Vec<Car>, Error> {
        sqlx::query_as!(
            Car,
            r#"
            SELECT id, brand_id, model_id, year, price, mileage, color, vin,
                   fuel_type as "fuel_type: _", transmission as "transmission: _",
                   status as "status: _", created_at, updated_at
            FROM cars
            WHERE status = $1
            ORDER BY created_at DESC
            "#,
            status as CarStatus
        )
            .fetch_all(&self.pool)
            .await
    }

    async fn find_by_brand_id(&self, brand_id: Uuid) -> Result<Vec<Car>, Error> {
        sqlx::query_as!(
            Car,
            r#"
            SELECT id, brand_id, model_id, year, price, mileage, color, vin,
                   fuel_type as "fuel_type: _", transmission as "transmission: _",
                   status as "status: _", created_at, updated_at
            FROM cars
            WHERE brand_id = $1
            ORDER BY created_at DESC
            "#,
            brand_id
        )
            .fetch_all(&self.pool)
            .await
    }

    async fn find_by_model_id(&self, model_id: Uuid) -> Result<Vec<Car>, Error> {
        sqlx::query_as!(
            Car,
            r#"
            SELECT id, brand_id, model_id, year, price, mileage, color, vin,
                   fuel_type as "fuel_type: _", transmission as "transmission: _",
                   status as "status: _", created_at, updated_at
            FROM cars
            WHERE model_id = $1
            ORDER BY created_at DESC
            "#,
            model_id
        )
            .fetch_all(&self.pool)
            .await
    }

    async fn find_by_vin(&self, vin: &str) -> Result<Option<Car>, Error> {
        sqlx::query_as!(
            Car,
            r#"
            SELECT id, brand_id, model_id, year, price, mileage, color, vin,
                   fuel_type as "fuel_type: _", transmission as "transmission: _",
                   status as "status: _", created_at, updated_at
            FROM cars
            WHERE vin = $1
            "#,
            vin
        )
            .fetch_optional(&self.pool)
            .await
    }

    async fn exists_by_vin(&self, vin: &str) -> Result<bool, Error> {
        let result = sqlx::query(
            "SELECT id FROM cars WHERE vin = $1 LIMIT 1"
        )
            .bind(vin)
            .fetch_optional(&self.pool)
            .await?;

        Ok(result.is_some())
    }

    async fn save(&self, create_request: &CreateCarRequest) -> Result<Car, Error> {
        let now = chrono::Utc::now();

        let fuel_type_str = match create_request.fuel_type {
            crate::models::FuelType::Petrol => "Petrol",
            crate::models::FuelType::Diesel => "Diesel",
            crate::models::FuelType::Electric => "Electric",
            crate::models::FuelType::Hybrid => "Hybrid",
        };

        let transmission_str = match create_request.transmission {
            crate::models::Transmission::Manual => "Manual",
            crate::models::Transmission::Automatic => "Automatic",
            crate::models::Transmission::CVT => "CVT",
        };

        let status_str = "Available";

        sqlx::query_as!(
            Car,
            r#"
            INSERT INTO cars (id, brand_id, model_id, year, price, mileage, color, vin,
                            fuel_type, transmission, status, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            RETURNING id, brand_id, model_id, year, price, mileage, color, vin,
                     fuel_type as "fuel_type: _", transmission as "transmission: _",
                     status as "status: _", created_at, updated_at
            "#,
            Uuid::new_v4(),
            create_request.brand_id,
            create_request.model_id,
            create_request.year,
            create_request.price,
            create_request.mileage,
            create_request.color,
            create_request.vin,
            fuel_type_str,
            transmission_str,
            status_str,
            now,
            now
        )
            .fetch_one(&self.pool)
            .await
    }

    async fn update(&self, id: Uuid, update_request: &UpdateCarRequest) -> Result<Option<Car>, Error> {
        let now = chrono::Utc::now();

        if let Some(car) = self.find_by_id(id).await? {
            let fuel_type = update_request.fuel_type.as_ref().unwrap_or(&car.fuel_type);
            let fuel_type_str = match fuel_type {
                crate::models::FuelType::Petrol => "Petrol",
                crate::models::FuelType::Diesel => "Diesel",
                crate::models::FuelType::Electric => "Electric",
                crate::models::FuelType::Hybrid => "Hybrid",
            };

            let transmission = update_request.transmission.as_ref().unwrap_or(&car.transmission);
            let transmission_str = match transmission {
                crate::models::Transmission::Manual => "Manual",
                crate::models::Transmission::Automatic => "Automatic",
                crate::models::Transmission::CVT => "CVT",
            };

            let status = update_request.status.as_ref().unwrap_or(&car.status);
            let status_str = match status {
                crate::models::CarStatus::Available => "Available",
                crate::models::CarStatus::Reserved => "Reserved",
                crate::models::CarStatus::Sold => "Sold",
                crate::models::CarStatus::Maintenance => "Maintenance",
            };

            let updated_car = sqlx::query_as!(
                Car,
                r#"
                UPDATE cars
                SET brand_id = $1, model_id = $2, year = $3, price = $4, mileage = $5,
                    color = $6, vin = $7, fuel_type = $8, transmission = $9, status = $10, updated_at = $11
                WHERE id = $12
                RETURNING id, brand_id, model_id, year, price, mileage, color, vin,
                         fuel_type as "fuel_type: _", transmission as "transmission: _",
                         status as "status: _", created_at, updated_at
                "#,
                update_request.brand_id.unwrap_or(car.brand_id),
                update_request.model_id.unwrap_or(car.model_id),
                update_request.year.unwrap_or(car.year),
                update_request.price.unwrap_or(car.price),
                update_request.mileage.unwrap_or(car.mileage),
                update_request.color.as_ref().unwrap_or(&car.color),
                update_request.vin.as_ref().unwrap_or(&car.vin),
                fuel_type_str,
                transmission_str,
                status_str,
                now,
                id
            )
                .fetch_optional(&self.pool)
                .await?;

            Ok(updated_car)
        } else {
            Ok(None)
        }
    }

    async fn delete(&self, id: Uuid) -> Result<bool, Error> {
        let result = sqlx::query(
            "DELETE FROM cars WHERE id = $1"
        )
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    async fn update_status(&self, id: Uuid, status: CarStatus) -> Result<Option<Car>, Error> {
        let now = chrono::Utc::now();

        let status_str = match status {
            CarStatus::Available => "Available",
            CarStatus::Reserved => "Reserved",
            CarStatus::Sold => "Sold",
            CarStatus::Maintenance => "Maintenance",
        };

        sqlx::query_as!(
            Car,
            r#"
            UPDATE cars
            SET status = $1, updated_at = $2
            WHERE id = $3
            RETURNING id, brand_id, model_id, year, price, mileage, color, vin,
                     fuel_type as "fuel_type: _", transmission as "transmission: _",
                     status as "status: _", created_at, updated_at
            "#,
            status_str,
            now,
            id
        )
            .fetch_optional(&self.pool)
            .await
    }
}