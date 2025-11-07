use async_trait::async_trait;
use sqlx::Error;
use uuid::Uuid;

use crate::models::{Car, CreateCarRequest, UpdateCarRequest, CarStatus, FuelType, Transmission, ServiceCampaign};
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

    // Новые методы для работы с сервисными кампаниями
    async fn add_completed_campaign(&self, car_id: Uuid, campaign_id: Uuid) -> Result<Option<Car>, Error>;
    async fn remove_completed_campaign(&self, car_id: Uuid, campaign_id: Uuid) -> Result<Option<Car>, Error>;
    async fn get_cars_by_completed_campaign(&self, campaign_id: Uuid) -> Result<Vec<Car>, Error>;
    async fn get_pending_campaigns_for_car(&self, car_id: Uuid) -> Result<Vec<ServiceCampaign>, Error>;
    async fn clear_completed_campaigns(&self, car_id: Uuid) -> Result<Option<Car>, Error>;
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
                   status as "status: _", completed_service_campaigns, created_at, updated_at
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
                   status as "status: _", completed_service_campaigns, created_at, updated_at
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
                   status as "status: _", completed_service_campaigns, created_at, updated_at
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
                   status as "status: _", completed_service_campaigns, created_at, updated_at
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
                   status as "status: _", completed_service_campaigns, created_at, updated_at
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
                   status as "status: _", completed_service_campaigns, created_at, updated_at
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
            FuelType::Petrol => "Petrol",
            FuelType::Diesel => "Diesel",
            FuelType::Electric => "Electric",
            FuelType::Hybrid => "Hybrid",
        };

        let transmission_str = match create_request.transmission {
            Transmission::Manual => "Manual",
            Transmission::Automatic => "Automatic",
            Transmission::CVT => "CVT",
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
                     status as "status: _", completed_service_campaigns, created_at, updated_at
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
                FuelType::Petrol => "Petrol",
                FuelType::Diesel => "Diesel",
                FuelType::Electric => "Electric",
                FuelType::Hybrid => "Hybrid",
            };

            let transmission = update_request.transmission.as_ref().unwrap_or(&car.transmission);
            let transmission_str = match transmission {
                Transmission::Manual => "Manual",
                Transmission::Automatic => "Automatic",
                Transmission::CVT => "CVT",
            };

            let status = update_request.status.as_ref().unwrap_or(&car.status);
            let status_str = match status {
                CarStatus::Available => "Available",
                CarStatus::Reserved => "Reserved",
                CarStatus::Sold => "Sold",
                CarStatus::Maintenance => "Maintenance",
            };

            let updated_car = sqlx::query_as!(
                Car,
                r#"
                UPDATE cars
                SET brand_id = $1, model_id = $2, year = $3, price = $4, mileage = $5,
                    color = $6, vin = $7, fuel_type = $8, transmission = $9, status = $10,
                    completed_service_campaigns = $11, updated_at = $12
                WHERE id = $13
                RETURNING id, brand_id, model_id, year, price, mileage, color, vin,
                         fuel_type as "fuel_type: _", transmission as "transmission: _",
                         status as "status: _", completed_service_campaigns, created_at, updated_at
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
                update_request.completed_service_campaigns.as_ref().unwrap_or(&car.completed_service_campaigns),
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
                     status as "status: _", completed_service_campaigns, created_at, updated_at
            "#,
            status_str,
            now,
            id
        )
            .fetch_optional(&self.pool)
            .await
    }

    // НОВЫЕ МЕТОДЫ ДЛЯ СЕРВИСНЫХ КАМПАНИЙ

    async fn add_completed_campaign(&self, car_id: Uuid, campaign_id: Uuid) -> Result<Option<Car>, Error> {
        let now = chrono::Utc::now();

        sqlx::query_as!(
            Car,
            r#"
            UPDATE cars
            SET completed_service_campaigns = array_append(completed_service_campaigns, $1),
                updated_at = $2
            WHERE id = $3
            AND NOT $1 = ANY(completed_service_campaigns)
            RETURNING id, brand_id, model_id, year, price, mileage, color, vin,
                     fuel_type as "fuel_type: _", transmission as "transmission: _",
                     status as "status: _", completed_service_campaigns, created_at, updated_at
            "#,
            campaign_id,
            now,
            car_id
        )
            .fetch_optional(&self.pool)
            .await
    }

    async fn remove_completed_campaign(&self, car_id: Uuid, campaign_id: Uuid) -> Result<Option<Car>, Error> {
        let now = chrono::Utc::now();

        sqlx::query_as!(
            Car,
            r#"
            UPDATE cars
            SET completed_service_campaigns = array_remove(completed_service_campaigns, $1),
                updated_at = $2
            WHERE id = $3
            RETURNING id, brand_id, model_id, year, price, mileage, color, vin,
                     fuel_type as "fuel_type: _", transmission as "transmission: _",
                     status as "status: _", completed_service_campaigns, created_at, updated_at
            "#,
            campaign_id,
            now,
            car_id
        )
            .fetch_optional(&self.pool)
            .await
    }

    async fn clear_completed_campaigns(&self, car_id: Uuid) -> Result<Option<Car>, Error> {
        let now = chrono::Utc::now();

        sqlx::query_as!(
            Car,
            r#"
            UPDATE cars
            SET completed_service_campaigns = '{}',
                updated_at = $1
            WHERE id = $2
            RETURNING id, brand_id, model_id, year, price, mileage, color, vin,
                     fuel_type as "fuel_type: _", transmission as "transmission: _",
                     status as "status: _", completed_service_campaigns, created_at, updated_at
            "#,
            now,
            car_id
        )
            .fetch_optional(&self.pool)
            .await
    }

    async fn get_cars_by_completed_campaign(&self, campaign_id: Uuid) -> Result<Vec<Car>, Error> {
        sqlx::query_as!(
            Car,
            r#"
            SELECT id, brand_id, model_id, year, price, mileage, color, vin,
                   fuel_type as "fuel_type: _", transmission as "transmission: _",
                   status as "status: _", completed_service_campaigns, created_at, updated_at
            FROM cars
            WHERE $1 = ANY(completed_service_campaigns)
            ORDER BY created_at DESC
            "#,
            campaign_id
        )
            .fetch_all(&self.pool)
            .await
    }

    async fn get_pending_campaigns_for_car(&self, car_id: Uuid) -> Result<Vec<ServiceCampaign>, Error> {
        // Получаем автомобиль
        let car = match self.find_by_id(car_id).await? {
            Some(car) => car,
            None => return Ok(Vec::new()),
        };

        // Используем query! для ручного маппинга
        let rows = sqlx::query!(
            r#"
            SELECT sc.id, sc.article, sc.name, sc.description, sc.brand_id, sc.car_model_id,
                   sc.target_vins, sc.required_parts, sc.required_works,
                   sc.is_mandatory, sc.is_completed,
                   sc.status, sc.created_at, sc.updated_at
            FROM service_campaigns sc
            WHERE sc.status = 'active'
            AND (sc.target_vins = '{}' OR $1 = ANY(sc.target_vins))
            AND sc.brand_id = $2
            AND sc.car_model_id = $3
            AND NOT sc.id = ANY($4)
            ORDER BY sc.is_mandatory DESC, sc.created_at DESC
            "#,
            car.vin,
            car.brand_id,
            car.model_id,
            &car.completed_service_campaigns
        )
            .fetch_all(&self.pool)
            .await?;

        // Ручное преобразование в ServiceCampaign
        let campaigns = rows.into_iter().map(|row| {
            let status = match row.status.as_str() {
                "active" => crate::models::ServiceCampaignStatus::Active,
                "completed" => crate::models::ServiceCampaignStatus::Completed,
                "cancelled" => crate::models::ServiceCampaignStatus::Cancelled,
                _ => crate::models::ServiceCampaignStatus::Active,
            };

            ServiceCampaign {
                id: row.id,
                article: row.article,
                name: row.name,
                description: row.description,
                brand_id: row.brand_id,
                car_model_id: row.car_model_id,
                target_vins: row.target_vins,
                required_parts: row.required_parts,
                required_works: row.required_works,
                is_mandatory: row.is_mandatory,
                is_completed: row.is_completed,
                status,
                created_at: row.created_at,
                updated_at: row.updated_at,
            }
        }).collect();

        Ok(campaigns)
    }
}