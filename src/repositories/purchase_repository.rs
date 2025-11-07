use async_trait::async_trait;
use sqlx::Error;
use uuid::Uuid;

use crate::models::{PurchaseRequest, CreatePurchaseRequest, RequestStatus};
use crate::database::DbPool;

#[async_trait]
pub trait PurchaseRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<PurchaseRequest>, Error>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<PurchaseRequest>, Error>;
    async fn find_by_customer_id(&self, customer_id: Uuid) -> Result<Vec<PurchaseRequest>, Error>;
    async fn find_by_car_id(&self, car_id: Uuid) -> Result<Vec<PurchaseRequest>, Error>;
    async fn find_by_status(&self, status: RequestStatus) -> Result<Vec<PurchaseRequest>, Error>;
    async fn save(&self, create_request: &CreatePurchaseRequest) -> Result<PurchaseRequest, Error>;
    async fn update_status(&self, id: Uuid, status: RequestStatus) -> Result<Option<PurchaseRequest>, Error>;
    async fn delete(&self, id: Uuid) -> Result<bool, Error>;
    async fn exists_by_car_and_customer(&self, car_id: Uuid, customer_id: Uuid) -> Result<bool, Error>;
}
#[derive(Clone)]
pub struct PurchaseRepositoryImpl {
    pool: DbPool,
}

impl PurchaseRepositoryImpl {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PurchaseRepository for PurchaseRepositoryImpl {
    async fn find_all(&self) -> Result<Vec<PurchaseRequest>, Error> {
        sqlx::query_as!(
            PurchaseRequest,
            r#"
            SELECT id, car_id, customer_id, status as "status: _",
                   offer_price, notes, created_at, updated_at
            FROM purchase_requests
            ORDER BY created_at DESC
            "#
        )
            .fetch_all(&self.pool)
            .await
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<PurchaseRequest>, Error> {
        sqlx::query_as!(
            PurchaseRequest,
            r#"
            SELECT id, car_id, customer_id, status as "status: _",
                   offer_price, notes, created_at, updated_at
            FROM purchase_requests
            WHERE id = $1
            "#,
            id
        )
            .fetch_optional(&self.pool)
            .await
    }

    async fn find_by_customer_id(&self, customer_id: Uuid) -> Result<Vec<PurchaseRequest>, Error> {
        sqlx::query_as!(
            PurchaseRequest,
            r#"
            SELECT id, car_id, customer_id, status as "status: _",
                   offer_price, notes, created_at, updated_at
            FROM purchase_requests
            WHERE customer_id = $1
            ORDER BY created_at DESC
            "#,
            customer_id
        )
            .fetch_all(&self.pool)
            .await
    }

    async fn find_by_car_id(&self, car_id: Uuid) -> Result<Vec<PurchaseRequest>, Error> {
        sqlx::query_as!(
            PurchaseRequest,
            r#"
            SELECT id, car_id, customer_id, status as "status: _",
                   offer_price, notes, created_at, updated_at
            FROM purchase_requests
            WHERE car_id = $1
            ORDER BY created_at DESC
            "#,
            car_id
        )
            .fetch_all(&self.pool)
            .await
    }

    async fn find_by_status(&self, status: RequestStatus) -> Result<Vec<PurchaseRequest>, Error> {
        sqlx::query_as!(
            PurchaseRequest,
            r#"
            SELECT id, car_id, customer_id, status as "status: _",
                   offer_price, notes, created_at, updated_at
            FROM purchase_requests
            WHERE status = $1
            ORDER BY created_at DESC
            "#,
            status as RequestStatus
        )
            .fetch_all(&self.pool)
            .await
    }

    async fn save(&self, create_request: &CreatePurchaseRequest) -> Result<PurchaseRequest, Error> {
        let now = chrono::Utc::now();

        sqlx::query_as!(
            PurchaseRequest,
            r#"
            INSERT INTO purchase_requests (id, car_id, customer_id, status, offer_price, notes, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, car_id, customer_id, status as "status: _",
                     offer_price, notes, created_at, updated_at
            "#,
            Uuid::new_v4(),
            create_request.car_id,
            create_request.customer_id,
            RequestStatus::Pending as RequestStatus,
            create_request.offer_price,
            create_request.notes,
            now,
            now
        )
            .fetch_one(&self.pool)
            .await
    }

    async fn update_status(&self, id: Uuid, status: RequestStatus) -> Result<Option<PurchaseRequest>, Error> {
        let now = chrono::Utc::now();

        sqlx::query_as!(
            PurchaseRequest,
            r#"
            UPDATE purchase_requests
            SET status = $1, updated_at = $2
            WHERE id = $3
            RETURNING id, car_id, customer_id, status as "status: _",
                     offer_price, notes, created_at, updated_at
            "#,
            status as RequestStatus,
            now,
            id
        )
            .fetch_optional(&self.pool)
            .await
    }

    async fn delete(&self, id: Uuid) -> Result<bool, Error> {
        let result = sqlx::query(
            "DELETE FROM purchase_requests WHERE id = $1"
        )
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    async fn exists_by_car_and_customer(&self, car_id: Uuid, customer_id: Uuid) -> Result<bool, Error> {
        let result = sqlx::query(
            "SELECT id FROM purchase_requests WHERE car_id = $1 AND customer_id = $2 LIMIT 1"
        )
            .bind(car_id)
            .bind(customer_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(result.is_some())
    }
}