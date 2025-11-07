use async_trait::async_trait;
use sqlx::Error;
use uuid::Uuid;

use crate::models::{CarModel, CreateCarModelRequest, UpdateCarModelRequest};
use crate::database::DbPool;

#[async_trait]
pub trait CarModelRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<CarModel>, Error>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<CarModel>, Error>;
    async fn find_by_brand_id(&self, brand_id: Uuid) -> Result<Vec<CarModel>, Error>;
    async fn find_by_name(&self, name: &str) -> Result<Vec<CarModel>, Error>;
    async fn exists_by_brand_and_name(&self, brand_id: Uuid, name: &str) -> Result<bool, Error>;
    async fn save(&self, create_request: &CreateCarModelRequest) -> Result<CarModel, Error>;
    async fn update(&self, id: Uuid, update_request: &UpdateCarModelRequest) -> Result<Option<CarModel>, Error>;
    async fn delete(&self, id: Uuid) -> Result<bool, Error>;
}

#[derive(Clone)]
pub struct CarModelRepositoryImpl {
    pool: DbPool,
}

impl CarModelRepositoryImpl {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CarModelRepository for CarModelRepositoryImpl {
    async fn find_all(&self) -> Result<Vec<CarModel>, Error> {
        sqlx::query_as!(
            CarModel,
            r#"
            SELECT id, name, brand_id, created_at, updated_at
            FROM car_models
            ORDER BY name
            "#
        )
            .fetch_all(&self.pool)
            .await
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<CarModel>, Error> {
        sqlx::query_as!(
            CarModel,
            r#"
            SELECT id, name, brand_id, created_at, updated_at
            FROM car_models
            WHERE id = $1
            "#,
            id
        )
            .fetch_optional(&self.pool)
            .await
    }

    async fn find_by_brand_id(&self, brand_id: Uuid) -> Result<Vec<CarModel>, Error> {
        sqlx::query_as!(
            CarModel,
            r#"
            SELECT id, name, brand_id, created_at, updated_at
            FROM car_models
            WHERE brand_id = $1
            ORDER BY name
            "#,
            brand_id
        )
            .fetch_all(&self.pool)
            .await
    }

    async fn find_by_name(&self, name: &str) -> Result<Vec<CarModel>, Error> {
        sqlx::query_as!(
            CarModel,
            r#"
            SELECT id, name, brand_id, created_at, updated_at
            FROM car_models
            WHERE name ILIKE $1
            ORDER BY name
            "#,
            format!("%{}%", name)
        )
            .fetch_all(&self.pool)
            .await
    }

    async fn exists_by_brand_and_name(&self, brand_id: Uuid, name: &str) -> Result<bool, Error> {
        let result = sqlx::query(
            "SELECT id FROM car_models WHERE brand_id = $1 AND name = $2 LIMIT 1"
        )
            .bind(brand_id)
            .bind(name)
            .fetch_optional(&self.pool)
            .await?;

        Ok(result.is_some())
    }

    async fn save(&self, create_request: &CreateCarModelRequest) -> Result<CarModel, Error> {
        let now = chrono::Utc::now();

        sqlx::query_as!(
            CarModel,
            r#"
            INSERT INTO car_models (id, name, brand_id, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, name, brand_id, created_at, updated_at
            "#,
            Uuid::new_v4(),
            create_request.name,
            create_request.brand_id,
            now,
            now
        )
            .fetch_one(&self.pool)
            .await
    }

    async fn update(&self, id: Uuid, update_request: &UpdateCarModelRequest) -> Result<Option<CarModel>, Error> {
        let now = chrono::Utc::now();

        if let Some(model) = self.find_by_id(id).await? {
            let updated_model = sqlx::query_as!(
                CarModel,
                r#"
                UPDATE car_models
                SET name = $1, brand_id = $2, updated_at = $3
                WHERE id = $4
                RETURNING id, name, brand_id, created_at, updated_at
                "#,
                update_request.name.as_ref().unwrap_or(&model.name),
                update_request.brand_id.unwrap_or(model.brand_id),
                now,
                id
            )
                .fetch_optional(&self.pool)
                .await?;

            Ok(updated_model)
        } else {
            Ok(None)
        }
    }

    async fn delete(&self, id: Uuid) -> Result<bool, Error> {
        let result = sqlx::query(
            "DELETE FROM car_models WHERE id = $1"
        )
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}