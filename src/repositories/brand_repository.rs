use async_trait::async_trait;
use sqlx::Error;
use uuid::Uuid;

use crate::models::{Brand, CreateBrandRequest, UpdateBrandRequest};
use crate::database::DbPool;

#[async_trait]
pub trait BrandRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Brand>, Error>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Brand>, Error>;
    async fn find_by_name(&self, name: &str) -> Result<Option<Brand>, Error>;
    async fn find_by_country(&self, country: &str) -> Result<Vec<Brand>, Error>;
    async fn exists_by_name(&self, name: &str) -> Result<bool, Error>;
    async fn save(&self, create_request: &CreateBrandRequest) -> Result<Brand, Error>;
    async fn update(&self, id: Uuid, update_request: &UpdateBrandRequest) -> Result<Option<Brand>, Error>;
    async fn delete(&self, id: Uuid) -> Result<bool, Error>;
}

#[derive(Clone)]
pub struct BrandRepositoryImpl {
    pool: DbPool,
}

impl BrandRepositoryImpl {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BrandRepository for BrandRepositoryImpl {
    async fn find_all(&self) -> Result<Vec<Brand>, Error> {
        sqlx::query_as!(
            Brand,
            r#"
            SELECT id, name, country, created_at, updated_at
            FROM brands
            ORDER BY name
            "#
        )
            .fetch_all(&self.pool)
            .await
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Brand>, Error> {
        sqlx::query_as!(
            Brand,
            r#"
            SELECT id, name, country, created_at, updated_at
            FROM brands
            WHERE id = $1
            "#,
            id
        )
            .fetch_optional(&self.pool)
            .await
    }

    async fn find_by_name(&self, name: &str) -> Result<Option<Brand>, Error> {
        sqlx::query_as!(
            Brand,
            r#"
            SELECT id, name, country, created_at, updated_at
            FROM brands
            WHERE name = $1
            "#,
            name
        )
            .fetch_optional(&self.pool)
            .await
    }

    async fn find_by_country(&self, country: &str) -> Result<Vec<Brand>, Error> {
        sqlx::query_as!(
            Brand,
            r#"
            SELECT id, name, country, created_at, updated_at
            FROM brands
            WHERE country ILIKE $1
            ORDER BY name
            "#,
            format!("%{}%", country)
        )
            .fetch_all(&self.pool)
            .await
    }

    async fn exists_by_name(&self, name: &str) -> Result<bool, Error> {
        let result = sqlx::query(
            "SELECT id FROM brands WHERE name = $1 LIMIT 1"
        )
            .bind(name)
            .fetch_optional(&self.pool)
            .await?;

        Ok(result.is_some())
    }

    async fn save(&self, create_request: &CreateBrandRequest) -> Result<Brand, Error> {
        let now = chrono::Utc::now();

        sqlx::query_as!(
            Brand,
            r#"
            INSERT INTO brands (id, name, country, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, name, country, created_at, updated_at
            "#,
            Uuid::new_v4(),
            create_request.name,
            create_request.country,
            now,
            now
        )
            .fetch_one(&self.pool)
            .await
    }

    async fn update(&self, id: Uuid, update_request: &UpdateBrandRequest) -> Result<Option<Brand>, Error> {
        let now = chrono::Utc::now();

        if let Some(brand) = self.find_by_id(id).await? {
            let updated_brand = sqlx::query_as!(
                Brand,
                r#"
                UPDATE brands
                SET name = $1, country = $2, updated_at = $3
                WHERE id = $4
                RETURNING id, name, country, created_at, updated_at
                "#,
                update_request.name.as_ref().unwrap_or(&brand.name),
                update_request.country.as_ref().unwrap_or(&brand.country),
                now,
                id
            )
                .fetch_optional(&self.pool)
                .await?;

            Ok(updated_brand)
        } else {
            Ok(None)
        }
    }

    async fn delete(&self, id: Uuid) -> Result<bool, Error> {
        let result = sqlx::query(
            "DELETE FROM brands WHERE id = $1"
        )
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}