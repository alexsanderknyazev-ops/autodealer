use actix_web::{web, HttpResponse};
use uuid::Uuid;
use validator::Validate;

use crate::{
    database::DbPool,
    models::{CreateCarModelRequest, UpdateCarModelRequest},
    repositories::car_model_repository::CarModelRepositoryImpl,
};
use crate::repositories::CarModelRepository;

// GET /api/car-models - получить все модели автомобилей
pub async fn get_car_models_handler(db_pool: web::Data<DbPool>) -> HttpResponse {
    let repo = CarModelRepositoryImpl::new(db_pool.get_ref().clone());
    match repo.find_all().await {
        Ok(models) => HttpResponse::Ok().json(models),
        Err(e) => {
            eprintln!("Error fetching car models: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch car models"
            }))
        }
    }
}

// GET /api/car-models/{id} - получить модель по ID
pub async fn get_car_model_by_id_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let repo = CarModelRepositoryImpl::new(db_pool.get_ref().clone());
    let id = path.into_inner();

    match repo.find_by_id(id).await {
        Ok(Some(model)) => HttpResponse::Ok().json(model),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Car model not found"
        })),
        Err(e) => {
            eprintln!("Error fetching car model {}: {}", id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch car model"
            }))
        }
    }
}

// GET /api/car-models/brand/{brand_id} - получить модели по бренду
pub async fn get_car_models_by_brand_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let repo = CarModelRepositoryImpl::new(db_pool.get_ref().clone());
    let brand_id = path.into_inner();

    match repo.find_by_brand_id(brand_id).await {
        Ok(models) => HttpResponse::Ok().json(models),
        Err(e) => {
            eprintln!("Error fetching car models by brand {}: {}", brand_id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch car models"
            }))
        }
    }
}

// GET /api/car-models/name/{name} - получить модели по названию
pub async fn get_car_models_by_name_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> HttpResponse {
    let repo = CarModelRepositoryImpl::new(db_pool.get_ref().clone());
    let name = path.into_inner();

    match repo.find_by_name(&name).await {
        Ok(models) => HttpResponse::Ok().json(models),
        Err(e) => {
            eprintln!("Error fetching car models by name {}: {}", name, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch car models"
            }))
        }
    }
}

// POST /api/car-models - создать модель автомобиля
pub async fn create_car_model_handler(
    db_pool: web::Data<DbPool>,
    create_request: web::Json<CreateCarModelRequest>,
) -> HttpResponse {
    let repo = CarModelRepositoryImpl::new(db_pool.get_ref().clone());

    if let Err(validation_errors) = create_request.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Validation failed",
            "details": validation_errors
        }));
    }
    match repo.exists_by_brand_and_name(create_request.brand_id, &create_request.name).await {
        Ok(true) => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Car model with this name already exists for this brand"
            }));
        }
        Err(e) => {
            eprintln!("Error checking car model: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to check car model"
            }));
        }
        _ => {}
    }

    match repo.save(&create_request).await {
        Ok(model) => HttpResponse::Created().json(model),
        Err(e) => {
            eprintln!("Error creating car model: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to create car model"
            }))
        }
    }
}

// PUT /api/car-models/{id} - обновить модель автомобиля
pub async fn update_car_model_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
    update_request: web::Json<UpdateCarModelRequest>,
) -> HttpResponse {
    let repo = CarModelRepositoryImpl::new(db_pool.get_ref().clone());
    let id = path.into_inner();

    if let Err(validation_errors) = update_request.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Validation failed",
            "details": validation_errors
        }));
    }
    if update_request.name.is_some() || update_request.brand_id.is_some() {
        let current_model = match repo.find_by_id(id).await {
            Ok(Some(model)) => model,
            Ok(None) => return HttpResponse::NotFound().json(serde_json::json!({
                "error": "Car model not found"
            })),
            Err(e) => {
                eprintln!("Error fetching car model: {}", e);
                return HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Failed to fetch car model"
                }));
            }
        };

        let new_name = update_request.name.as_ref().unwrap_or(&current_model.name);
        let new_brand_id = update_request.brand_id.unwrap_or(current_model.brand_id);
        if new_name != &current_model.name || new_brand_id != current_model.brand_id {
            match repo.exists_by_brand_and_name(new_brand_id, new_name).await {
                Ok(true) => {
                    return HttpResponse::BadRequest().json(serde_json::json!({
                        "error": "Car model with this name already exists for this brand"
                    }));
                }
                Err(e) => {
                    eprintln!("Error checking car model: {}", e);
                    return HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": "Failed to check car model"
                    }));
                }
                _ => {}
            }
        }
    }

    match repo.update(id, &update_request).await {
        Ok(Some(model)) => HttpResponse::Ok().json(model),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Car model not found"
        })),
        Err(e) => {
            eprintln!("Error updating car model {}: {}", id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to update car model"
            }))
        }
    }
}

// DELETE /api/car-models/{id} - удалить модель автомобиля
pub async fn delete_car_model_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let repo = CarModelRepositoryImpl::new(db_pool.get_ref().clone());
    let id = path.into_inner();

    match repo.delete(id).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Car model not found"
        })),
        Err(e) => {
            eprintln!("Error deleting car model {}: {}", id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to delete car model"
            }))
        }
    }
}