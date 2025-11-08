use async_trait::async_trait;
use sqlx::Error;
use uuid::Uuid;

use crate::models::warehouse::{
    WarehouseItem, WarehouseItemWithPart, CreateWarehouseItemRequest,
    UpdateWarehouseItemRequest, StockMovementRequest, StockMovementType
};
use crate::database::DbPool;

#[async_trait]
pub trait WarehouseRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<WarehouseItemWithPart>, Error>;
    async fn find_all_with_low_stock(&self) -> Result<Vec<WarehouseItemWithPart>, Error>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<WarehouseItemWithPart>, Error>;
    async fn find_by_part_id(&self, part_id: Uuid) -> Result<Option<WarehouseItem>, Error>;
    async fn find_by_article(&self, article: &str) -> Result<Option<WarehouseItemWithPart>, Error>;
    async fn find_by_location(&self, location: &str) -> Result<Vec<WarehouseItemWithPart>, Error>;
    async fn exists_by_part_id(&self, part_id: Uuid) -> Result<bool, Error>;
    async fn save(&self, create_request: &CreateWarehouseItemRequest) -> Result<WarehouseItem, Error>;
    async fn update(&self, id: Uuid, update_request: &UpdateWarehouseItemRequest) -> Result<Option<WarehouseItem>, Error>;
    async fn delete(&self, id: Uuid) -> Result<bool, Error>;
    async fn update_stock(&self, part_id: Uuid, movement_request: &StockMovementRequest) -> Result<Option<WarehouseItem>, Error>;
    async fn get_total_value(&self) -> Result<f64, Error>;
}

#[derive(Clone)]
pub struct WarehouseRepositoryImpl {
    pool: DbPool,
}

impl WarehouseRepositoryImpl {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl WarehouseRepository for WarehouseRepositoryImpl {
    async fn find_all(&self) -> Result<Vec<WarehouseItemWithPart>, Error> {
        sqlx::query_as!(
            WarehouseItemWithPart,
            r#"
            SELECT
                w.id, w.part_id, w.quantity, w.min_stock_level, w.max_stock_level,
                w.location, w.created_at, w.updated_at,
                p.article as part_article, p.name as part_name
            FROM warehouse w
            JOIN parts p ON w.part_id = p.id
            ORDER BY p.article
            "#
        )
            .fetch_all(&self.pool)
            .await
    }

    async fn find_all_with_low_stock(&self) -> Result<Vec<WarehouseItemWithPart>, Error> {
        sqlx::query_as!(
            WarehouseItemWithPart,
            r#"
            SELECT
                w.id, w.part_id, w.quantity, w.min_stock_level, w.max_stock_level,
                w.location, w.created_at, w.updated_at,
                p.article as part_article, p.name as part_name
            FROM warehouse w
            JOIN parts p ON w.part_id = p.id
            WHERE w.quantity <= w.min_stock_level
            ORDER BY w.quantity ASC
            "#
        )
            .fetch_all(&self.pool)
            .await
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<WarehouseItemWithPart>, Error> {
        sqlx::query_as!(
            WarehouseItemWithPart,
            r#"
            SELECT
                w.id, w.part_id, w.quantity, w.min_stock_level, w.max_stock_level,
                w.location, w.created_at, w.updated_at,
                p.article as part_article, p.name as part_name
            FROM warehouse w
            JOIN parts p ON w.part_id = p.id
            WHERE w.id = $1
            "#,
            id
        )
            .fetch_optional(&self.pool)
            .await
    }

    async fn find_by_part_id(&self, part_id: Uuid) -> Result<Option<WarehouseItem>, Error> {
        sqlx::query_as!(
            WarehouseItem,
            r#"
            SELECT id, part_id, quantity, min_stock_level, max_stock_level,
                   location, created_at, updated_at
            FROM warehouse
            WHERE part_id = $1
            "#,
            part_id
        )
            .fetch_optional(&self.pool)
            .await
    }

    async fn find_by_article(&self, article: &str) -> Result<Option<WarehouseItemWithPart>, Error> {
        sqlx::query_as!(
            WarehouseItemWithPart,
            r#"
            SELECT
                w.id, w.part_id, w.quantity, w.min_stock_level, w.max_stock_level,
                w.location, w.created_at, w.updated_at,
                p.article as part_article, p.name as part_name
            FROM warehouse w
            JOIN parts p ON w.part_id = p.id
            WHERE p.article = $1
            "#,
            article
        )
            .fetch_optional(&self.pool)
            .await
    }

    async fn find_by_location(&self, location: &str) -> Result<Vec<WarehouseItemWithPart>, Error> {
        sqlx::query_as!(
            WarehouseItemWithPart,
            r#"
            SELECT
                w.id, w.part_id, w.quantity, w.min_stock_level, w.max_stock_level,
                w.location, w.created_at, w.updated_at,
                p.article as part_article, p.name as part_name
            FROM warehouse w
            JOIN parts p ON w.part_id = p.id
            WHERE w.location ILIKE $1
            ORDER BY p.article
            "#,
            format!("%{}%", location)
        )
            .fetch_all(&self.pool)
            .await
    }

    async fn exists_by_part_id(&self, part_id: Uuid) -> Result<bool, Error> {
        let result = sqlx::query(
            "SELECT id FROM warehouse WHERE part_id = $1 LIMIT 1"
        )
            .bind(part_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(result.is_some())
    }

    async fn save(&self, create_request: &CreateWarehouseItemRequest) -> Result<WarehouseItem, Error> {
        let now = chrono::Utc::now();

        sqlx::query_as!(
            WarehouseItem,
            r#"
            INSERT INTO warehouse (id, part_id, quantity, min_stock_level, max_stock_level, location, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, part_id, quantity, min_stock_level, max_stock_level, location, created_at, updated_at
            "#,
            Uuid::new_v4(),
            create_request.part_id,
            create_request.quantity,
            create_request.min_stock_level.unwrap_or(0),
            create_request.max_stock_level.unwrap_or(100),
            create_request.location,
            now,
            now
        )
            .fetch_one(&self.pool)
            .await
    }

    async fn update(&self, id: Uuid, update_request: &UpdateWarehouseItemRequest) -> Result<Option<WarehouseItem>, Error> {
        let now = chrono::Utc::now();

        if let Some(item) = self.find_by_id(id).await? {
            let updated_item = sqlx::query_as!(
                WarehouseItem,
                r#"
                UPDATE warehouse
                SET quantity = $1, min_stock_level = $2, max_stock_level = $3,
                    location = $4, updated_at = $5
                WHERE id = $6
                RETURNING id, part_id, quantity, min_stock_level, max_stock_level,
                         location, created_at, updated_at
                "#,
                update_request.quantity.unwrap_or(item.quantity),
                update_request.min_stock_level.unwrap_or(item.min_stock_level),
                update_request.max_stock_level.unwrap_or(item.max_stock_level),
                update_request.location.as_ref().or(item.location.as_ref()),
                now,
                id
            )
                .fetch_optional(&self.pool)
                .await?;

            Ok(updated_item)
        } else {
            Ok(None)
        }
    }

    async fn delete(&self, id: Uuid) -> Result<bool, Error> {
        let result = sqlx::query(
            "DELETE FROM warehouse WHERE id = $1"
        )
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    async fn update_stock(&self, part_id: Uuid, movement_request: &StockMovementRequest) -> Result<Option<WarehouseItem>, Error> {
        let now = chrono::Utc::now();

        let new_quantity = match movement_request.movement_type {
            StockMovementType::Incoming => {
                sqlx::query!(
                    "UPDATE warehouse SET quantity = quantity + $1, updated_at = $2 WHERE part_id = $3",
                    movement_request.quantity,
                    now,
                    part_id
                )
            }
            StockMovementType::Outgoing => {
                sqlx::query!(
                    "UPDATE warehouse SET quantity = quantity - $1, updated_at = $2 WHERE part_id = $3 AND quantity >= $1",
                    movement_request.quantity,
                    now,
                    part_id
                )
            }
            StockMovementType::Adjustment => {
                sqlx::query!(
                    "UPDATE warehouse SET quantity = $1, updated_at = $2 WHERE part_id = $3",
                    movement_request.quantity,
                    now,
                    part_id
                )
            }
        }
            .execute(&self.pool)
            .await?;

        if new_quantity.rows_affected() > 0 {
            self.find_by_part_id(part_id).await
        } else {
            Ok(None)
        }
    }

    async fn get_total_value(&self) -> Result<f64, Error> {
        let result = sqlx::query!(
            r#"
            SELECT SUM(w.quantity * p.purchase_price) as total_value
            FROM warehouse w
            JOIN parts p ON w.part_id = p.id
            "#
        )
            .fetch_one(&self.pool)
            .await?;

        Ok(result.total_value.unwrap_or(0.0))
    }
}