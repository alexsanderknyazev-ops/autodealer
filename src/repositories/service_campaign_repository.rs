use async_trait::async_trait;
use sqlx::{Error, Row};
use uuid::Uuid;

use crate::models::{ServiceCampaign, CreateServiceCampaignRequest, UpdateServiceCampaignRequest, ServiceCampaignStatus};
use crate::database::DbPool;

#[async_trait]
pub trait ServiceCampaignRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<ServiceCampaign>, Error>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<ServiceCampaign>, Error>;
    async fn find_by_article(&self, article: &str) -> Result<Option<ServiceCampaign>, Error>;
    async fn find_by_brand(&self, brand_id: Uuid) -> Result<Vec<ServiceCampaign>, Error>;
    async fn find_by_car_model(&self, car_model_id: Uuid) -> Result<Vec<ServiceCampaign>, Error>;
    async fn find_by_status(&self, status: ServiceCampaignStatus) -> Result<Vec<ServiceCampaign>, Error>;
    async fn find_by_mandatory(&self, is_mandatory: bool) -> Result<Vec<ServiceCampaign>, Error>;
    async fn find_by_completed(&self, is_completed: bool) -> Result<Vec<ServiceCampaign>, Error>;
    async fn find_by_vin(&self, vin: &str) -> Result<Vec<ServiceCampaign>, Error>;
    async fn exists_by_article(&self, article: &str) -> Result<bool, Error>;
    async fn save(&self, create_request: &CreateServiceCampaignRequest) -> Result<ServiceCampaign, Error>;
    async fn update(&self, id: Uuid, update_request: &UpdateServiceCampaignRequest) -> Result<Option<ServiceCampaign>, Error>;
    async fn delete(&self, id: Uuid) -> Result<bool, Error>;
    async fn update_status(&self, id: Uuid, status: ServiceCampaignStatus) -> Result<Option<ServiceCampaign>, Error>;
    async fn mark_completed(&self, id: Uuid) -> Result<Option<ServiceCampaign>, Error>;
    async fn mark_pending(&self, id: Uuid) -> Result<Option<ServiceCampaign>, Error>;
}

#[derive(Clone)]
pub struct ServiceCampaignRepositoryImpl {
    pool: DbPool,
}

impl ServiceCampaignRepositoryImpl {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    // Вспомогательная функция для преобразования строки в ServiceCampaignStatus
    fn status_from_str(status: &str) -> ServiceCampaignStatus {
        match status.to_lowercase().as_str() {
            "active" => ServiceCampaignStatus::Active,
            "completed" => ServiceCampaignStatus::Completed,
            "cancelled" => ServiceCampaignStatus::Cancelled,
            _ => ServiceCampaignStatus::Active, // default
        }
    }

    // Вспомогательная функция для создания ServiceCampaign из row
    fn campaign_from_row(&self, row: sqlx::postgres::PgRow) -> Result<ServiceCampaign, Error> {
        let target_vins: Vec<String> = row.try_get("target_vins")?;
        let required_parts: Vec<Uuid> = row.try_get("required_parts")?;
        let required_works: Vec<Uuid> = row.try_get("required_works")?;
        let status_str: String = row.try_get("status")?;

        Ok(ServiceCampaign {
            id: row.try_get("id")?,
            article: row.try_get("article")?,
            name: row.try_get("name")?,
            description: row.try_get("description")?,
            brand_id: row.try_get("brand_id")?,
            car_model_id: row.try_get("car_model_id")?,
            target_vins,
            required_parts,
            required_works,
            is_mandatory: row.try_get("is_mandatory")?,
            is_completed: row.try_get("is_completed")?,
            status: Self::status_from_str(&status_str),
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }
}

#[async_trait]
impl ServiceCampaignRepository for ServiceCampaignRepositoryImpl {
    async fn find_all(&self) -> Result<Vec<ServiceCampaign>, Error> {
        let rows = sqlx::query(
            r#"
            SELECT id, article, name, description, brand_id, car_model_id,
                   target_vins, required_parts, required_works,
                   is_mandatory, is_completed,
                   status, created_at, updated_at
            FROM service_campaigns
            ORDER BY created_at DESC
            "#
        )
            .fetch_all(&self.pool)
            .await?;

        let mut campaigns = Vec::new();
        for row in rows {
            campaigns.push(self.campaign_from_row(row)?);
        }
        Ok(campaigns)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<ServiceCampaign>, Error> {
        let row = sqlx::query(
            r#"
            SELECT id, article, name, description, brand_id, car_model_id,
                   target_vins, required_parts, required_works,
                   is_mandatory, is_completed,
                   status, created_at, updated_at
            FROM service_campaigns
            WHERE id = $1
            "#
        )
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        match row {
            Some(row) => Ok(Some(self.campaign_from_row(row)?)),
            None => Ok(None),
        }
    }

    async fn find_by_article(&self, article: &str) -> Result<Option<ServiceCampaign>, Error> {
        let row = sqlx::query(
            r#"
            SELECT id, article, name, description, brand_id, car_model_id,
                   target_vins, required_parts, required_works,
                   is_mandatory, is_completed,
                   status, created_at, updated_at
            FROM service_campaigns
            WHERE article = $1
            "#
        )
            .bind(article)
            .fetch_optional(&self.pool)
            .await?;

        match row {
            Some(row) => Ok(Some(self.campaign_from_row(row)?)),
            None => Ok(None),
        }
    }

    async fn find_by_brand(&self, brand_id: Uuid) -> Result<Vec<ServiceCampaign>, Error> {
        let rows = sqlx::query(
            r#"
            SELECT id, article, name, description, brand_id, car_model_id,
                   target_vins, required_parts, required_works,
                   is_mandatory, is_completed,
                   status, created_at, updated_at
            FROM service_campaigns
            WHERE brand_id = $1
            ORDER BY created_at DESC
            "#
        )
            .bind(brand_id)
            .fetch_all(&self.pool)
            .await?;

        let mut campaigns = Vec::new();
        for row in rows {
            campaigns.push(self.campaign_from_row(row)?);
        }
        Ok(campaigns)
    }

    async fn find_by_car_model(&self, car_model_id: Uuid) -> Result<Vec<ServiceCampaign>, Error> {
        let rows = sqlx::query(
            r#"
            SELECT id, article, name, description, brand_id, car_model_id,
                   target_vins, required_parts, required_works,
                   is_mandatory, is_completed,
                   status, created_at, updated_at
            FROM service_campaigns
            WHERE car_model_id = $1
            ORDER BY created_at DESC
            "#
        )
            .bind(car_model_id)
            .fetch_all(&self.pool)
            .await?;

        let mut campaigns = Vec::new();
        for row in rows {
            campaigns.push(self.campaign_from_row(row)?);
        }
        Ok(campaigns)
    }

    async fn find_by_status(&self, status: ServiceCampaignStatus) -> Result<Vec<ServiceCampaign>, Error> {
        let status_str = match status {
            ServiceCampaignStatus::Active => "active",
            ServiceCampaignStatus::Completed => "completed",
            ServiceCampaignStatus::Cancelled => "cancelled",
        };

        let rows = sqlx::query(
            r#"
            SELECT id, article, name, description, brand_id, car_model_id,
                   target_vins, required_parts, required_works,
                   is_mandatory, is_completed,
                   status, created_at, updated_at
            FROM service_campaigns
            WHERE status = $1
            ORDER BY created_at DESC
            "#
        )
            .bind(status_str)
            .fetch_all(&self.pool)
            .await?;

        let mut campaigns = Vec::new();
        for row in rows {
            campaigns.push(self.campaign_from_row(row)?);
        }
        Ok(campaigns)
    }

    async fn find_by_mandatory(&self, is_mandatory: bool) -> Result<Vec<ServiceCampaign>, Error> {
        let rows = sqlx::query(
            r#"
            SELECT id, article, name, description, brand_id, car_model_id,
                   target_vins, required_parts, required_works,
                   is_mandatory, is_completed,
                   status, created_at, updated_at
            FROM service_campaigns
            WHERE is_mandatory = $1
            ORDER BY created_at DESC
            "#
        )
            .bind(is_mandatory)
            .fetch_all(&self.pool)
            .await?;

        let mut campaigns = Vec::new();
        for row in rows {
            campaigns.push(self.campaign_from_row(row)?);
        }
        Ok(campaigns)
    }

    async fn find_by_completed(&self, is_completed: bool) -> Result<Vec<ServiceCampaign>, Error> {
        let rows = sqlx::query(
            r#"
            SELECT id, article, name, description, brand_id, car_model_id,
                   target_vins, required_parts, required_works,
                   is_mandatory, is_completed,
                   status, created_at, updated_at
            FROM service_campaigns
            WHERE is_completed = $1
            ORDER BY created_at DESC
            "#
        )
            .bind(is_completed)
            .fetch_all(&self.pool)
            .await?;

        let mut campaigns = Vec::new();
        for row in rows {
            campaigns.push(self.campaign_from_row(row)?);
        }
        Ok(campaigns)
    }

    async fn find_by_vin(&self, vin: &str) -> Result<Vec<ServiceCampaign>, Error> {
        let rows = sqlx::query(
            r#"
            SELECT id, article, name, description, brand_id, car_model_id,
                   target_vins, required_parts, required_works,
                   is_mandatory, is_completed,
                   status, created_at, updated_at
            FROM service_campaigns
            WHERE $1 = ANY(target_vins)
            ORDER BY created_at DESC
            "#
        )
            .bind(vin)
            .fetch_all(&self.pool)
            .await?;

        let mut campaigns = Vec::new();
        for row in rows {
            campaigns.push(self.campaign_from_row(row)?);
        }
        Ok(campaigns)
    }

    async fn exists_by_article(&self, article: &str) -> Result<bool, Error> {
        let result = sqlx::query(
            "SELECT id FROM service_campaigns WHERE article = $1 LIMIT 1"
        )
            .bind(article)
            .fetch_optional(&self.pool)
            .await?;

        Ok(result.is_some())
    }

    async fn save(&self, create_request: &CreateServiceCampaignRequest) -> Result<ServiceCampaign, Error> {
        let now = chrono::Utc::now();
        let id = Uuid::new_v4();

        let row = sqlx::query(
            r#"
            INSERT INTO service_campaigns (id, article, name, description, brand_id, car_model_id,
                                         target_vins, required_parts, required_works,
                                         is_mandatory, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            RETURNING id, article, name, description, brand_id, car_model_id,
                     target_vins, required_parts, required_works,
                     is_mandatory, is_completed,
                     status, created_at, updated_at
            "#
        )
            .bind(id)
            .bind(&create_request.article)
            .bind(&create_request.name)
            .bind(&create_request.description)
            .bind(create_request.brand_id)
            .bind(create_request.car_model_id)
            .bind(&create_request.target_vins)
            .bind(&create_request.required_parts)
            .bind(&create_request.required_works)
            .bind(create_request.is_mandatory)
            .bind(now)
            .bind(now)
            .fetch_one(&self.pool)
            .await?;

        self.campaign_from_row(row)
    }

    async fn update(&self, id: Uuid, update_request: &UpdateServiceCampaignRequest) -> Result<Option<ServiceCampaign>, Error> {
        let now = chrono::Utc::now();

        if let Some(current_campaign) = self.find_by_id(id).await? {
            let status = update_request.status.as_ref().unwrap_or(&current_campaign.status);
            let status_str = match status {
                ServiceCampaignStatus::Active => "active",
                ServiceCampaignStatus::Completed => "completed",
                ServiceCampaignStatus::Cancelled => "cancelled",
            };

            let row = sqlx::query(
                r#"
                UPDATE service_campaigns
                SET article = $1, name = $2, description = $3, brand_id = $4, car_model_id = $5,
                    target_vins = $6, required_parts = $7, required_works = $8,
                    is_mandatory = $9, is_completed = $10, status = $11, updated_at = $12
                WHERE id = $13
                RETURNING id, article, name, description, brand_id, car_model_id,
                         target_vins, required_parts, required_works,
                         is_mandatory, is_completed,
                         status, created_at, updated_at
                "#
            )
                .bind(update_request.article.as_ref().unwrap_or(&current_campaign.article))
                .bind(update_request.name.as_ref().unwrap_or(&current_campaign.name))
                .bind(update_request.description.as_ref().or(current_campaign.description.as_ref()))
                .bind(update_request.brand_id.unwrap_or(current_campaign.brand_id))
                .bind(update_request.car_model_id.unwrap_or(current_campaign.car_model_id))
                .bind(update_request.target_vins.as_ref().unwrap_or(&current_campaign.target_vins))
                .bind(update_request.required_parts.as_ref().unwrap_or(&current_campaign.required_parts))
                .bind(update_request.required_works.as_ref().unwrap_or(&current_campaign.required_works))
                .bind(update_request.is_mandatory.unwrap_or(current_campaign.is_mandatory))
                .bind(update_request.is_completed.unwrap_or(current_campaign.is_completed))
                .bind(status_str)
                .bind(now)
                .bind(id)
                .fetch_optional(&self.pool)
                .await?;

            match row {
                Some(row) => Ok(Some(self.campaign_from_row(row)?)),
                None => Ok(None),
            }
        } else {
            Ok(None)
        }
    }

    async fn delete(&self, id: Uuid) -> Result<bool, Error> {
        let result = sqlx::query(
            "DELETE FROM service_campaigns WHERE id = $1"
        )
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    async fn update_status(&self, id: Uuid, status: ServiceCampaignStatus) -> Result<Option<ServiceCampaign>, Error> {
        let now = chrono::Utc::now();

        let status_str = match status {
            ServiceCampaignStatus::Active => "active",
            ServiceCampaignStatus::Completed => "completed",
            ServiceCampaignStatus::Cancelled => "cancelled",
        };

        let row = sqlx::query(
            r#"
            UPDATE service_campaigns
            SET status = $1, updated_at = $2
            WHERE id = $3
            RETURNING id, article, name, description, brand_id, car_model_id,
                     target_vins, required_parts, required_works,
                     is_mandatory, is_completed,
                     status, created_at, updated_at
            "#
        )
            .bind(status_str)
            .bind(now)
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        match row {
            Some(row) => Ok(Some(self.campaign_from_row(row)?)),
            None => Ok(None),
        }
    }

    async fn mark_completed(&self, id: Uuid) -> Result<Option<ServiceCampaign>, Error> {
        let now = chrono::Utc::now();

        let row = sqlx::query(
            r#"
            UPDATE service_campaigns
            SET is_completed = true, status = 'completed', updated_at = $1
            WHERE id = $2
            RETURNING id, article, name, description, brand_id, car_model_id,
                     target_vins, required_parts, required_works,
                     is_mandatory, is_completed,
                     status, created_at, updated_at
            "#
        )
            .bind(now)
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        match row {
            Some(row) => Ok(Some(self.campaign_from_row(row)?)),
            None => Ok(None),
        }
    }

    async fn mark_pending(&self, id: Uuid) -> Result<Option<ServiceCampaign>, Error> {
        let now = chrono::Utc::now();

        let row = sqlx::query(
            r#"
            UPDATE service_campaigns
            SET is_completed = false, status = 'active', updated_at = $1
            WHERE id = $2
            RETURNING id, article, name, description, brand_id, car_model_id,
                     target_vins, required_parts, required_works,
                     is_mandatory, is_completed,
                     status, created_at, updated_at
            "#
        )
            .bind(now)
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        match row {
            Some(row) => Ok(Some(self.campaign_from_row(row)?)),
            None => Ok(None),
        }
    }
}