use actix_web::{web, HttpResponse};
use uuid::Uuid;
use validator::Validate;

use crate::{
    database::DbPool,
    models::{CreatePartRequest, UpdatePartRequest},
    repositories::part_repository::PartRepositoryImpl,
};
use crate::repositories::PartRepository;

// GET /api/parts - получить все запчасти
pub async fn get_parts_handler(db_pool: web::Data<DbPool>) -> HttpResponse {
    let repo = PartRepositoryImpl::new(db_pool.get_ref().clone());
    match repo.find_all().await {
        Ok(parts) => HttpResponse::Ok().json(parts),
        Err(e) => {
            eprintln!("Error fetching parts: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch parts"
            }))
        }
    }
}

// GET /api/parts/{id} - получить запчасть по ID
pub async fn get_part_by_id_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let repo = PartRepositoryImpl::new(db_pool.get_ref().clone());
    let id = path.into_inner();

    match repo.find_by_id(id).await {
        Ok(Some(part)) => HttpResponse::Ok().json(part),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Part not found"
        })),
        Err(e) => {
            eprintln!("Error fetching part {}: {}", id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch part"
            }))
        }
    }
}

// GET /api/parts/article/{article} - получить запчасть по артикулу
pub async fn get_part_by_article_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> HttpResponse {
    let repo = PartRepositoryImpl::new(db_pool.get_ref().clone());
    let article = path.into_inner();

    match repo.find_by_article(&article).await {
        Ok(Some(part)) => HttpResponse::Ok().json(part),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Part not found"
        })),
        Err(e) => {
            eprintln!("Error fetching part by article {}: {}", article, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch part"
            }))
        }
    }
}

// GET /api/parts/brand/{brand_id} - получить запчасти по бренду
pub async fn get_parts_by_brand_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let repo = PartRepositoryImpl::new(db_pool.get_ref().clone());
    let brand_id = path.into_inner();

    match repo.find_by_brand(brand_id).await {
        Ok(parts) => HttpResponse::Ok().json(parts),
        Err(e) => {
            eprintln!("Error fetching parts by brand {}: {}", brand_id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch parts"
            }))
        }
    }
}

// GET /api/parts/car-model/{car_model_id} - получить запчасти по модели автомобиля
pub async fn get_parts_by_car_model_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let repo = PartRepositoryImpl::new(db_pool.get_ref().clone());
    let car_model_id = path.into_inner();

    match repo.find_by_car_model(car_model_id).await {
        Ok(parts) => HttpResponse::Ok().json(parts),
        Err(e) => {
            eprintln!("Error fetching parts by car model {}: {}", car_model_id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch parts"
            }))
        }
    }
}

// GET /api/parts/vin/{vin} - получить запчасти по VIN коду
pub async fn get_parts_by_vin_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> HttpResponse {
    let repo = PartRepositoryImpl::new(db_pool.get_ref().clone());
    let vin = path.into_inner();

    match repo.find_by_vin(&vin).await {
        Ok(parts) => HttpResponse::Ok().json(parts),
        Err(e) => {
            eprintln!("Error fetching parts by VIN {}: {}", vin, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch parts"
            }))
        }
    }
}

// POST /api/parts - создать запчасть
pub async fn create_part_handler(
    db_pool: web::Data<DbPool>,
    create_request: web::Json<CreatePartRequest>,
) -> HttpResponse {
    let repo = PartRepositoryImpl::new(db_pool.get_ref().clone());

    if let Err(validation_errors) = create_request.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Validation failed",
            "details": validation_errors
        }));
    }
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
        Ok(part) => HttpResponse::Created().json(part),
        Err(e) => {
            eprintln!("Error creating part: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to create part"
            }))
        }
    }
}

// PUT /api/parts/{id} - обновить запчасть
pub async fn update_part_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
    update_request: web::Json<UpdatePartRequest>,
) -> HttpResponse {
    let repo = PartRepositoryImpl::new(db_pool.get_ref().clone());
    let id = path.into_inner();

    if let Err(validation_errors) = update_request.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Validation failed",
            "details": validation_errors
        }));
    }

    match repo.update(id, &update_request).await {
        Ok(Some(part)) => HttpResponse::Ok().json(part),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Part not found"
        })),
        Err(e) => {
            eprintln!("Error updating part {}: {}", id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to update part"
            }))
        }
    }
}

// DELETE /api/parts/{id} - удалить запчасть
pub async fn delete_part_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let repo = PartRepositoryImpl::new(db_pool.get_ref().clone());
    let id = path.into_inner();

    match repo.delete(id).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Part not found"
        })),
        Err(e) => {
            eprintln!("Error deleting part {}: {}", id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to delete part"
            }))
        }
    }
}