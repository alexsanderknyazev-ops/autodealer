use actix_web::{web, HttpResponse};
use uuid::Uuid;
use validator::Validate;

use crate::{
    database::DbPool,
    models::{CreateServiceCampaignRequest, UpdateServiceCampaignRequest, ServiceCampaignStatus},
    repositories::service_campaign_repository::ServiceCampaignRepositoryImpl,
};
use crate::repositories::service_campaign_repository::ServiceCampaignRepository;

// GET /api/service-campaigns - получить все сервисные кампании
pub async fn get_service_campaigns_handler(db_pool: web::Data<DbPool>) -> HttpResponse {
    let repo = ServiceCampaignRepositoryImpl::new(db_pool.get_ref().clone());
    match repo.find_all().await {
        Ok(campaigns) => HttpResponse::Ok().json(campaigns),
        Err(e) => {
            eprintln!("Error fetching service campaigns: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch service campaigns"
            }))
        }
    }
}

// GET /api/service-campaigns/{id} - получить сервисную кампанию по ID
pub async fn get_service_campaign_by_id_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let repo = ServiceCampaignRepositoryImpl::new(db_pool.get_ref().clone());
    let id = path.into_inner();

    match repo.find_by_id(id).await {
        Ok(Some(campaign)) => HttpResponse::Ok().json(campaign),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Service campaign not found"
        })),
        Err(e) => {
            eprintln!("Error fetching service campaign {}: {}", id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch service campaign"
            }))
        }
    }
}

// GET /api/service-campaigns/article/{article} - получить сервисную кампанию по артикулу
pub async fn get_service_campaign_by_article_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> HttpResponse {
    let repo = ServiceCampaignRepositoryImpl::new(db_pool.get_ref().clone());
    let article = path.into_inner();

    match repo.find_by_article(&article).await {
        Ok(Some(campaign)) => HttpResponse::Ok().json(campaign),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Service campaign not found"
        })),
        Err(e) => {
            eprintln!("Error fetching service campaign by article {}: {}", article, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch service campaign"
            }))
        }
    }
}

// GET /api/service-campaigns/brand/{brand_id} - получить сервисные кампании по бренду
pub async fn get_service_campaigns_by_brand_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let repo = ServiceCampaignRepositoryImpl::new(db_pool.get_ref().clone());
    let brand_id = path.into_inner();

    match repo.find_by_brand(brand_id).await {
        Ok(campaigns) => HttpResponse::Ok().json(campaigns),
        Err(e) => {
            eprintln!("Error fetching service campaigns by brand {}: {}", brand_id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch service campaigns"
            }))
        }
    }
}

// GET /api/service-campaigns/car-model/{car_model_id} - получить сервисные кампании по модели автомобиля
pub async fn get_service_campaigns_by_car_model_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let repo = ServiceCampaignRepositoryImpl::new(db_pool.get_ref().clone());
    let car_model_id = path.into_inner();

    match repo.find_by_car_model(car_model_id).await {
        Ok(campaigns) => HttpResponse::Ok().json(campaigns),
        Err(e) => {
            eprintln!("Error fetching service campaigns by car model {}: {}", car_model_id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch service campaigns"
            }))
        }
    }
}

// GET /api/service-campaigns/status/{status} - получить сервисные кампании по статусу
pub async fn get_service_campaigns_by_status_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> HttpResponse {
    let repo = ServiceCampaignRepositoryImpl::new(db_pool.get_ref().clone());
    let status_str = path.into_inner();

    let status = match status_str.to_lowercase().as_str() {
        "active" => ServiceCampaignStatus::Active,
        "completed" => ServiceCampaignStatus::Completed,
        "cancelled" => ServiceCampaignStatus::Cancelled,
        _ => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid status. Use: active, completed, or cancelled"
            }))
        }
    };

    match repo.find_by_status(status).await {
        Ok(campaigns) => HttpResponse::Ok().json(campaigns),
        Err(e) => {
            eprintln!("Error fetching service campaigns by status {}: {}", status_str, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch service campaigns"
            }))
        }
    }
}

// GET /api/service-campaigns/mandatory/{is_mandatory} - получить сервисные кампании по обязательности
pub async fn get_service_campaigns_by_mandatory_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<bool>,
) -> HttpResponse {
    let repo = ServiceCampaignRepositoryImpl::new(db_pool.get_ref().clone());
    let is_mandatory = path.into_inner();

    match repo.find_by_mandatory(is_mandatory).await {
        Ok(campaigns) => HttpResponse::Ok().json(campaigns),
        Err(e) => {
            eprintln!("Error fetching service campaigns by mandatory {}: {}", is_mandatory, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch service campaigns"
            }))
        }
    }
}

// GET /api/service-campaigns/completed/{is_completed} - получить сервисные кампании по выполнению
pub async fn get_service_campaigns_by_completed_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<bool>,
) -> HttpResponse {
    let repo = ServiceCampaignRepositoryImpl::new(db_pool.get_ref().clone());
    let is_completed = path.into_inner();

    match repo.find_by_completed(is_completed).await {
        Ok(campaigns) => HttpResponse::Ok().json(campaigns),
        Err(e) => {
            eprintln!("Error fetching service campaigns by completed {}: {}", is_completed, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch service campaigns"
            }))
        }
    }
}

// GET /api/service-campaigns/vin/{vin} - получить сервисные кампании по VIN коду
pub async fn get_service_campaigns_by_vin_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> HttpResponse {
    let repo = ServiceCampaignRepositoryImpl::new(db_pool.get_ref().clone());
    let vin = path.into_inner();

    match repo.find_by_vin(&vin).await {
        Ok(campaigns) => HttpResponse::Ok().json(campaigns),
        Err(e) => {
            eprintln!("Error fetching service campaigns by VIN {}: {}", vin, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch service campaigns"
            }))
        }
    }
}

// POST /api/service-campaigns - создать сервисную кампанию
pub async fn create_service_campaign_handler(
    db_pool: web::Data<DbPool>,
    create_request: web::Json<CreateServiceCampaignRequest>,
) -> HttpResponse {
    let repo = ServiceCampaignRepositoryImpl::new(db_pool.get_ref().clone());

    if let Err(validation_errors) = create_request.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Validation failed",
            "details": validation_errors
        }));
    }

    // Проверка уникальности артикула
    match repo.exists_by_article(&create_request.article).await {
        Ok(true) => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Article already exists"
            }));
        }
        Err(e) => {
            eprintln!("Error checking article: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to check article"
            }));
        }
        _ => {}
    }

    match repo.save(&create_request).await {
        Ok(campaign) => HttpResponse::Created().json(campaign),
        Err(e) => {
            eprintln!("Error creating service campaign: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to create service campaign"
            }))
        }
    }
}

// PUT /api/service-campaigns/{id} - обновить сервисную кампанию
pub async fn update_service_campaign_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
    update_request: web::Json<UpdateServiceCampaignRequest>,
) -> HttpResponse {
    let repo = ServiceCampaignRepositoryImpl::new(db_pool.get_ref().clone());
    let id = path.into_inner();

    if let Err(validation_errors) = update_request.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Validation failed",
            "details": validation_errors
        }));
    }

    // Если обновляется артикул, проверяем уникальность
    if let Some(new_article) = &update_request.article {
        match repo.exists_by_article(new_article).await {
            Ok(true) => {
                return HttpResponse::BadRequest().json(serde_json::json!({
                    "error": "Article already exists"
                }));
            }
            Err(e) => {
                eprintln!("Error checking article: {}", e);
                return HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Failed to check article"
                }));
            }
            _ => {}
        }
    }

    match repo.update(id, &update_request).await {
        Ok(Some(campaign)) => HttpResponse::Ok().json(campaign),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Service campaign not found"
        })),
        Err(e) => {
            eprintln!("Error updating service campaign {}: {}", id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to update service campaign"
            }))
        }
    }
}

// DELETE /api/service-campaigns/{id} - удалить сервисную кампанию
pub async fn delete_service_campaign_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let repo = ServiceCampaignRepositoryImpl::new(db_pool.get_ref().clone());
    let id = path.into_inner();

    match repo.delete(id).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Service campaign not found"
        })),
        Err(e) => {
            eprintln!("Error deleting service campaign {}: {}", id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to delete service campaign"
            }))
        }
    }
}

// PATCH /api/service-campaigns/{id}/status - обновить статус сервисной кампании
pub async fn update_service_campaign_status_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
    status: web::Json<String>,
) -> HttpResponse {
    let repo = ServiceCampaignRepositoryImpl::new(db_pool.get_ref().clone());
    let id = path.into_inner();
    let status_str = status.into_inner();

    let campaign_status = match status_str.to_lowercase().as_str() {
        "active" => ServiceCampaignStatus::Active,
        "completed" => ServiceCampaignStatus::Completed,
        "cancelled" => ServiceCampaignStatus::Cancelled,
        _ => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid status. Use: active, completed, or cancelled"
            }))
        }
    };

    match repo.update_status(id, campaign_status).await {
        Ok(Some(campaign)) => HttpResponse::Ok().json(campaign),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Service campaign not found"
        })),
        Err(e) => {
            eprintln!("Error updating service campaign status {}: {}", id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to update service campaign status"
            }))
        }
    }
}

// PATCH /api/service-campaigns/{id}/complete - отметить сервисную кампанию как выполненную
pub async fn mark_service_campaign_completed_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let repo = ServiceCampaignRepositoryImpl::new(db_pool.get_ref().clone());
    let id = path.into_inner();

    match repo.mark_completed(id).await {
        Ok(Some(campaign)) => HttpResponse::Ok().json(campaign),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Service campaign not found"
        })),
        Err(e) => {
            eprintln!("Error marking service campaign as completed {}: {}", id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to mark service campaign as completed"
            }))
        }
    }
}

// PATCH /api/service-campaigns/{id}/pending - отметить сервисную кампанию как ожидающую
pub async fn mark_service_campaign_pending_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let repo = ServiceCampaignRepositoryImpl::new(db_pool.get_ref().clone());
    let id = path.into_inner();

    match repo.mark_pending(id).await {
        Ok(Some(campaign)) => HttpResponse::Ok().json(campaign),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Service campaign not found"
        })),
        Err(e) => {
            eprintln!("Error marking service campaign as pending {}: {}", id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to mark service campaign as pending"
            }))
        }
    }
}