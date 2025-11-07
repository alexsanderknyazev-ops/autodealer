use async_trait::async_trait;
use sqlx::Error;
use uuid::Uuid;

use crate::models::{Work, CreateWorkRequest, UpdateWorkRequest};
use crate::database::DbPool;

#[async_trait]
pub trait WorkRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Work>, Error>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Work>, Error>;
    async fn find_by_article(&self, article: &str) -> Result<Option<Work>, Error>;
    async fn find_by_brand(&self, brand_id: Uuid) -> Result<Vec<Work>, Error>;
    async fn find_by_car_model(&self, car_model_id: Uuid) -> Result<Vec<Work>, Error>;
    async fn find_by_name(&self, name: &str) -> Result<Vec<Work>, Error>;
    async fn exists_by_article(&self, article: &str) -> Result<bool, Error>;
    async fn save(&self, create_request: &CreateWorkRequest) -> Result<Work, Error>;
    async fn update(&self, id: Uuid, update_request: &UpdateWorkRequest) -> Result<Option<Work>, Error>;
    async fn delete(&self, id: Uuid) -> Result<bool, Error>;
}

#[derive(Clone)]
pub struct WorkRepositoryImpl {
    pool: DbPool,
}

impl WorkRepositoryImpl {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl WorkRepository for WorkRepositoryImpl {
    async fn find_all(&self) -> Result<Vec<Work>, Error> {
        sqlx::query_as!(
            Work,
            r#"
            SELECT id, name, article, norm_hours, brand_id, car_model_id, created_at, updated_at
            FROM works
            ORDER BY name
            "#
        )
            .fetch_all(&self.pool)
            .await
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Work>, Error> {
        sqlx::query_as!(
            Work,
            r#"
            SELECT id, name, article, norm_hours, brand_id, car_model_id, created_at, updated_at
            FROM works
            WHERE id = $1
            "#,
            id
        )
            .fetch_optional(&self.pool)
            .await
    }

    async fn find_by_article(&self, article: &str) -> Result<Option<Work>, Error> {
        sqlx::query_as!(
            Work,
            r#"
            SELECT id, name, article, norm_hours, brand_id, car_model_id, created_at, updated_at
            FROM works
            WHERE article = $1
            "#,
            article
        )
            .fetch_optional(&self.pool)
            .await
    }

    async fn find_by_brand(&self, brand_id: Uuid) -> Result<Vec<Work>, Error> {
        sqlx::query_as!(
            Work,
            r#"
            SELECT id, name, article, norm_hours, brand_id, car_model_id, created_at, updated_at
            FROM works
            WHERE brand_id = $1
            ORDER BY name
            "#,
            brand_id
        )
            .fetch_all(&self.pool)
            .await
    }

    async fn find_by_car_model(&self, car_model_id: Uuid) -> Result<Vec<Work>, Error> {
        sqlx::query_as!(
            Work,
            r#"
            SELECT id, name, article, norm_hours, brand_id, car_model_id, created_at, updated_at
            FROM works
            WHERE car_model_id = $1
            ORDER BY name
            "#,
            car_model_id
        )
            .fetch_all(&self.pool)
            .await
    }

    async fn find_by_name(&self, name: &str) -> Result<Vec<Work>, Error> {
        sqlx::query_as!(
            Work,
            r#"
            SELECT id, name, article, norm_hours, brand_id, car_model_id, created_at, updated_at
            FROM works
            WHERE name ILIKE $1
            ORDER BY name
            "#,
            format!("%{}%", name)
        )
            .fetch_all(&self.pool)
            .await
    }

    async fn exists_by_article(&self, article: &str) -> Result<bool, Error> {
        let result = sqlx::query(
            "SELECT id FROM works WHERE article = $1 LIMIT 1"
        )
            .bind(article)
            .fetch_optional(&self.pool)
            .await?;

        Ok(result.is_some())
    }

    async fn save(&self, create_request: &CreateWorkRequest) -> Result<Work, Error> {
        let now = chrono::Utc::now();

        sqlx::query_as!(
            Work,
            r#"
            INSERT INTO works (id, name, article, norm_hours, brand_id, car_model_id, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, name, article, norm_hours, brand_id, car_model_id, created_at, updated_at
            "#,
            Uuid::new_v4(),
            create_request.name,
            create_request.article,
            create_request.norm_hours,
            create_request.brand_id,
            create_request.car_model_id,
            now,
            now
        )
            .fetch_one(&self.pool)
            .await
    }

    async fn update(&self, id: Uuid, update_request: &UpdateWorkRequest) -> Result<Option<Work>, Error> {
        let now = chrono::Utc::now();

        if let Some(work) = self.find_by_id(id).await? {
            let updated_work = sqlx::query_as!(
                Work,
                r#"
                UPDATE works
                SET name = $1, article = $2, norm_hours = $3, brand_id = $4, car_model_id = $5, updated_at = $6
                WHERE id = $7
                RETURNING id, name, article, norm_hours, brand_id, car_model_id, created_at, updated_at
                "#,
                update_request.name.as_ref().unwrap_or(&work.name),
                update_request.article.as_ref().unwrap_or(&work.article),
                update_request.norm_hours.unwrap_or(work.norm_hours),
                update_request.brand_id.unwrap_or(work.brand_id),
                update_request.car_model_id.unwrap_or(work.car_model_id),
                now,
                id
            )
                .fetch_optional(&self.pool)
                .await?;

            Ok(updated_work)
        } else {
            Ok(None)
        }
    }

    async fn delete(&self, id: Uuid) -> Result<bool, Error> {
        let result = sqlx::query(
            "DELETE FROM works WHERE id = $1"
        )
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}