use actix_web::{web, HttpResponse};
use uuid::Uuid;
use validator::Validate;

use crate::{
    database::DbPool,
    models::{CreateBrandRequest, UpdateBrandRequest},
    repositories::brand_repository::BrandRepositoryImpl,
};
use crate::repositories::BrandRepository;

// GET /api/brands - получить все бренды
pub async fn get_brands_handler(db_pool: web::Data<DbPool>) -> HttpResponse {
    let repo = BrandRepositoryImpl::new(db_pool.get_ref().clone());
    match repo.find_all().await {
        Ok(brands) => HttpResponse::Ok().json(brands),
        Err(e) => {
            eprintln!("Error fetching brands: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch brands"
            }))
        }
    }
}

// GET /api/brands/{id} - получить бренд по ID
pub async fn get_brand_by_id_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let repo = BrandRepositoryImpl::new(db_pool.get_ref().clone());
    let id = path.into_inner();

    match repo.find_by_id(id).await {
        Ok(Some(brand)) => HttpResponse::Ok().json(brand),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Brand not found"
        })),
        Err(e) => {
            eprintln!("Error fetching brand {}: {}", id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch brand"
            }))
        }
    }
}

// GET /api/brands/name/{name} - получить бренд по названию
pub async fn get_brand_by_name_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> HttpResponse {
    let repo = BrandRepositoryImpl::new(db_pool.get_ref().clone());
    let name = path.into_inner();

    match repo.find_by_name(&name).await {
        Ok(Some(brand)) => HttpResponse::Ok().json(brand),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Brand not found"
        })),
        Err(e) => {
            eprintln!("Error fetching brand by name {}: {}", name, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch brand"
            }))
        }
    }
}

// GET /api/brands/country/{country} - получить бренды по стране
pub async fn get_brands_by_country_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> HttpResponse {
    let repo = BrandRepositoryImpl::new(db_pool.get_ref().clone());
    let country = path.into_inner();

    match repo.find_by_country(&country).await {
        Ok(brands) => HttpResponse::Ok().json(brands),
        Err(e) => {
            eprintln!("Error fetching brands by country {}: {}", country, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch brands"
            }))
        }
    }
}

// POST /api/brands - создать бренд
pub async fn create_brand_handler(
    db_pool: web::Data<DbPool>,
    create_request: web::Json<CreateBrandRequest>,
) -> HttpResponse {
    let repo = BrandRepositoryImpl::new(db_pool.get_ref().clone());

    if let Err(validation_errors) = create_request.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Validation failed",
            "details": validation_errors
        }));
    }

    // Проверка уникальности названия бренда
    match repo.exists_by_name(&create_request.name).await {
        Ok(true) => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Brand name already exists"
            }));
        }
        Err(e) => {
            eprintln!("Error checking brand name: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to check brand name"
            }));
        }
        _ => {}
    }

    match repo.save(&create_request).await {
        Ok(brand) => HttpResponse::Created().json(brand),
        Err(e) => {
            eprintln!("Error creating brand: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to create brand"
            }))
        }
    }
}

// PUT /api/brands/{id} - обновить бренд
pub async fn update_brand_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
    update_request: web::Json<UpdateBrandRequest>,
) -> HttpResponse {
    let repo = BrandRepositoryImpl::new(db_pool.get_ref().clone());
    let id = path.into_inner();

    if let Err(validation_errors) = update_request.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Validation failed",
            "details": validation_errors
        }));
    }

    // Если обновляется название, проверяем уникальность
    if let Some(new_name) = &update_request.name {
        match repo.exists_by_name(new_name).await {
            Ok(true) => {
                return HttpResponse::BadRequest().json(serde_json::json!({
                    "error": "Brand name already exists"
                }));
            }
            Err(e) => {
                eprintln!("Error checking brand name: {}", e);
                return HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Failed to check brand name"
                }));
            }
            _ => {}
        }
    }

    match repo.update(id, &update_request).await {
        Ok(Some(brand)) => HttpResponse::Ok().json(brand),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Brand not found"
        })),
        Err(e) => {
            eprintln!("Error updating brand {}: {}", id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to update brand"
            }))
        }
    }
}

// DELETE /api/brands/{id} - удалить бренд
pub async fn delete_brand_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let repo = BrandRepositoryImpl::new(db_pool.get_ref().clone());
    let id = path.into_inner();

    match repo.delete(id).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Brand not found"
        })),
        Err(e) => {
            eprintln!("Error deleting brand {}: {}", id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to delete brand"
            }))
        }
    }
}