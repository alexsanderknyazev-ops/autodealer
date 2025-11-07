use async_trait::async_trait;
use sqlx::Error;
use uuid::Uuid;

use crate::models::{Customer, CreateCustomerRequest};
use crate::database::DbPool;

#[async_trait]
pub trait CustomerRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Customer>, Error>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Customer>, Error>;
    async fn find_by_email(&self, email: &str) -> Result<Option<Customer>, Error>;
    async fn find_by_name(&self, first_name: &str, last_name: &str) -> Result<Vec<Customer>, Error>;
    async fn save(&self, create_request: &CreateCustomerRequest) -> Result<Customer, Error>;
    async fn update(&self, id: Uuid, update_request: &CreateCustomerRequest) -> Result<Option<Customer>, Error>;
    async fn delete(&self, id: Uuid) -> Result<bool, Error>;
    async fn exists_by_email(&self, email: &str) -> Result<bool, Error>;
}
#[derive(Clone)]
pub struct CustomerRepositoryImpl {
    pool: DbPool,
}

impl CustomerRepositoryImpl {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CustomerRepository for CustomerRepositoryImpl {
    async fn find_all(&self) -> Result<Vec<Customer>, Error> {
        sqlx::query_as!(
            Customer,
            r#"
            SELECT id, first_name, last_name, email, phone, created_at
            FROM customers
            ORDER BY created_at DESC
            "#
        )
            .fetch_all(&self.pool)
            .await
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Customer>, Error> {
        sqlx::query_as!(
            Customer,
            r#"
            SELECT id, first_name, last_name, email, phone, created_at
            FROM customers
            WHERE id = $1
            "#,
            id
        )
            .fetch_optional(&self.pool)
            .await
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<Customer>, Error> {
        sqlx::query_as!(
            Customer,
            r#"
            SELECT id, first_name, last_name, email, phone, created_at
            FROM customers
            WHERE email = $1
            "#,
            email
        )
            .fetch_optional(&self.pool)
            .await
    }

    async fn find_by_name(&self, first_name: &str, last_name: &str) -> Result<Vec<Customer>, Error> {
        sqlx::query_as!(
            Customer,
            r#"
            SELECT id, first_name, last_name, email, phone, created_at
            FROM customers
            WHERE first_name ILIKE $1 AND last_name ILIKE $2
            ORDER BY created_at DESC
            "#,
            format!("%{}%", first_name),
            format!("%{}%", last_name)
        )
            .fetch_all(&self.pool)
            .await
    }

    async fn save(&self, create_request: &CreateCustomerRequest) -> Result<Customer, Error> {
        let now = chrono::Utc::now();

        sqlx::query_as!(
            Customer,
            r#"
            INSERT INTO customers (id, first_name, last_name, email, phone, created_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, first_name, last_name, email, phone, created_at
            "#,
            Uuid::new_v4(),
            create_request.first_name,
            create_request.last_name,
            create_request.email,
            create_request.phone,
            now
        )
            .fetch_one(&self.pool)
            .await
    }

    async fn update(&self, id: Uuid, update_request: &CreateCustomerRequest) -> Result<Option<Customer>, Error> {
        sqlx::query_as!(
            Customer,
            r#"
            UPDATE customers
            SET first_name = $1, last_name = $2, email = $3, phone = $4
            WHERE id = $5
            RETURNING id, first_name, last_name, email, phone, created_at
            "#,
            update_request.first_name,
            update_request.last_name,
            update_request.email,
            update_request.phone,
            id
        )
            .fetch_optional(&self.pool)
            .await
    }

    async fn delete(&self, id: Uuid) -> Result<bool, Error> {
        let result = sqlx::query(
            "DELETE FROM customers WHERE id = $1"
        )
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    async fn exists_by_email(&self, email: &str) -> Result<bool, Error> {
        let result = sqlx::query(
            "SELECT id FROM customers WHERE email = $1 LIMIT 1"
        )
            .bind(email)
            .fetch_optional(&self.pool)
            .await?;

        Ok(result.is_some())
    }
}