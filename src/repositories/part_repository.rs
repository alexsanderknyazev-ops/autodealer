use async_trait::async_trait;
use sqlx::Error;
use uuid::Uuid;

use crate::models::{Part, CreatePartRequest, UpdatePartRequest};
use crate::database::DbPool;

#[async_trait]
pub trait PartRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Part>, Error>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Part>, Error>;
    async fn find_by_article(&self, article: &str) -> Result<Option<Part>, Error>;
    async fn find_by_brand(&self, brand_id: Uuid) -> Result<Vec<Part>, Error>;
    async fn find_by_car_model(&self, car_model_id: Uuid) -> Result<Vec<Part>, Error>;
    async fn find_by_vin(&self, vin: &str) -> Result<Vec<Part>, Error>;
    async fn exists_by_article(&self, article: &str) -> Result<bool, Error>;
    async fn save(&self, create_request: &CreatePartRequest) -> Result<Part, Error>;
    async fn update(&self, id: Uuid, update_request: &UpdatePartRequest) -> Result<Option<Part>, Error>;
    async fn delete(&self, id: Uuid) -> Result<bool, Error>;
}

#[derive(Clone)]
pub struct PartRepositoryImpl {
    pool: DbPool,
}

impl PartRepositoryImpl {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PartRepository for PartRepositoryImpl {
    async fn find_all(&self) -> Result<Vec<Part>, Error> {
        let parts = sqlx::query!(
            r#"
            SELECT id, article, name, brand_id, car_model_id, purchase_price, sale_price,
                   compatible_vins, created_at, updated_at
            FROM parts
            ORDER BY created_at DESC
            "#
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(parts.into_iter().map(|row| Part {
            id: row.id,
            article: row.article,
            name: row.name,
            brand_id: row.brand_id.unwrap(), // Если может быть NULL, добавьте обработку ошибки
            car_model_id: row.car_model_id.unwrap(), // Если может быть NULL, добавьте обработку ошибки
            purchase_price: row.purchase_price,
            sale_price: row.sale_price,
            compatible_vins: row.compatible_vins, // Убрали unwrap_or_default()
            created_at: row.created_at,
            updated_at: row.updated_at,
        }).collect())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Part>, Error> {
        let row = sqlx::query!(
            r#"
            SELECT id, article, name, brand_id, car_model_id, purchase_price, sale_price,
                   compatible_vins, created_at, updated_at
            FROM parts
            WHERE id = $1
            "#,
            id
        )
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(|row| Part {
            id: row.id,
            article: row.article,
            name: row.name,
            brand_id: row.brand_id.unwrap(),
            car_model_id: row.car_model_id.unwrap(),
            purchase_price: row.purchase_price,
            sale_price: row.sale_price,
            compatible_vins: row.compatible_vins, // Убрали unwrap_or_default()
            created_at: row.created_at,
            updated_at: row.updated_at,
        }))
    }

    async fn find_by_article(&self, article: &str) -> Result<Option<Part>, Error> {
        let row = sqlx::query!(
            r#"
            SELECT id, article, name, brand_id, car_model_id, purchase_price, sale_price,
                   compatible_vins, created_at, updated_at
            FROM parts
            WHERE article = $1
            "#,
            article
        )
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(|row| Part {
            id: row.id,
            article: row.article,
            name: row.name,
            brand_id: row.brand_id.unwrap(),
            car_model_id: row.car_model_id.unwrap(),
            purchase_price: row.purchase_price,
            sale_price: row.sale_price,
            compatible_vins: row.compatible_vins, // Убрали unwrap_or_default()
            created_at: row.created_at,
            updated_at: row.updated_at,
        }))
    }

    async fn find_by_brand(&self, brand_id: Uuid) -> Result<Vec<Part>, Error> {
        let parts = sqlx::query!(
            r#"
            SELECT id, article, name, brand_id, car_model_id, purchase_price, sale_price,
                   compatible_vins, created_at, updated_at
            FROM parts
            WHERE brand_id = $1
            ORDER BY created_at DESC
            "#,
            brand_id
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(parts.into_iter().map(|row| Part {
            id: row.id,
            article: row.article,
            name: row.name,
            brand_id: row.brand_id.unwrap(),
            car_model_id: row.car_model_id.unwrap(),
            purchase_price: row.purchase_price,
            sale_price: row.sale_price,
            compatible_vins: row.compatible_vins, // Убрали unwrap_or_default()
            created_at: row.created_at,
            updated_at: row.updated_at,
        }).collect())
    }

    async fn find_by_car_model(&self, car_model_id: Uuid) -> Result<Vec<Part>, Error> {
        let parts = sqlx::query!(
            r#"
            SELECT id, article, name, brand_id, car_model_id, purchase_price, sale_price,
                   compatible_vins, created_at, updated_at
            FROM parts
            WHERE car_model_id = $1
            ORDER BY created_at DESC
            "#,
            car_model_id
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(parts.into_iter().map(|row| Part {
            id: row.id,
            article: row.article,
            name: row.name,
            brand_id: row.brand_id.unwrap(),
            car_model_id: row.car_model_id.unwrap(),
            purchase_price: row.purchase_price,
            sale_price: row.sale_price,
            compatible_vins: row.compatible_vins, // Убрали unwrap_or_default()
            created_at: row.created_at,
            updated_at: row.updated_at,
        }).collect())
    }

    async fn find_by_vin(&self, vin: &str) -> Result<Vec<Part>, Error> {
        let parts = sqlx::query!(
            r#"
            SELECT id, article, name, brand_id, car_model_id, purchase_price, sale_price,
                   compatible_vins, created_at, updated_at
            FROM parts
            WHERE $1 = ANY(compatible_vins)
            ORDER BY created_at DESC
            "#,
            vin
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(parts.into_iter().map(|row| Part {
            id: row.id,
            article: row.article,
            name: row.name,
            brand_id: row.brand_id.unwrap(),
            car_model_id: row.car_model_id.unwrap(),
            purchase_price: row.purchase_price,
            sale_price: row.sale_price,
            compatible_vins: row.compatible_vins, // Убрали unwrap_or_default()
            created_at: row.created_at,
            updated_at: row.updated_at,
        }).collect())
    }

    async fn exists_by_article(&self, article: &str) -> Result<bool, Error> {
        let result = sqlx::query(
            "SELECT id FROM parts WHERE article = $1 LIMIT 1"
        )
            .bind(article)
            .fetch_optional(&self.pool)
            .await?;

        Ok(result.is_some())
    }

    async fn save(&self, create_request: &CreatePartRequest) -> Result<Part, Error> {
        let now = chrono::Utc::now();
        let id = Uuid::new_v4();

        let row = sqlx::query!(
            r#"
            INSERT INTO parts (id, article, name, brand_id, car_model_id, purchase_price, sale_price,
                             compatible_vins, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING id, article, name, brand_id, car_model_id, purchase_price, sale_price,
                     compatible_vins, created_at, updated_at
            "#,
            id,
            create_request.article,
            create_request.name,
            create_request.brand_id,
            create_request.car_model_id,
            create_request.purchase_price,
            create_request.sale_price,
            &create_request.compatible_vins,
            now,
            now
        )
            .fetch_one(&self.pool)
            .await?;

        Ok(Part {
            id: row.id,
            article: row.article,
            name: row.name,
            brand_id: row.brand_id.unwrap(),
            car_model_id: row.car_model_id.unwrap(),
            purchase_price: row.purchase_price,
            sale_price: row.sale_price,
            compatible_vins: row.compatible_vins, // Убрали unwrap_or_default()
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
    }

    async fn update(&self, id: Uuid, update_request: &UpdatePartRequest) -> Result<Option<Part>, Error> {
        let now = chrono::Utc::now();

        // Сначала получаем текущую запчасть
        if let Some(current_part) = self.find_by_id(id).await? {
            // Подготавливаем значения для обновления
            let article = update_request.article.as_ref().unwrap_or(&current_part.article);
            let name = update_request.name.as_ref().unwrap_or(&current_part.name);
            let brand_id = update_request.brand_id.unwrap_or(current_part.brand_id);
            let car_model_id = update_request.car_model_id.unwrap_or(current_part.car_model_id);
            let purchase_price = update_request.purchase_price.unwrap_or(current_part.purchase_price);
            let sale_price = update_request.sale_price.unwrap_or(current_part.sale_price);
            let compatible_vins = update_request.compatible_vins.as_ref().unwrap_or(&current_part.compatible_vins);

            let row = sqlx::query!(
                r#"
                UPDATE parts
                SET article = $1, name = $2, brand_id = $3, car_model_id = $4, purchase_price = $5,
                    sale_price = $6, compatible_vins = $7, updated_at = $8
                WHERE id = $9
                RETURNING id, article, name, brand_id, car_model_id, purchase_price, sale_price,
                         compatible_vins, created_at, updated_at
                "#,
                article,
                name,
                brand_id,
                car_model_id,
                purchase_price,
                sale_price,
                compatible_vins,
                now,
                id
            )
                .fetch_optional(&self.pool)
                .await?;

            Ok(row.map(|row| Part {
                id: row.id,
                article: row.article,
                name: row.name,
                brand_id: row.brand_id.unwrap(),
                car_model_id: row.car_model_id.unwrap(),
                purchase_price: row.purchase_price,
                sale_price: row.sale_price,
                compatible_vins: row.compatible_vins, // Убрали unwrap_or_default()
                created_at: row.created_at,
                updated_at: row.updated_at,
            }))
        } else {
            Ok(None)
        }
    }

    async fn delete(&self, id: Uuid) -> Result<bool, Error> {
        let result = sqlx::query(
            "DELETE FROM parts WHERE id = $1"
        )
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}