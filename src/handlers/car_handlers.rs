use actix_web::{web, HttpResponse};
use uuid::Uuid;
use validator::Validate;

use crate::{
    database::DbPool,
    models::{CarStatus, CreateCarRequest, UpdateCarRequest},
    repositories::car_repository::CarRepositoryImpl,
};
use crate::repositories::CarRepository;

// GET /api/cars - получить все автомобили
pub async fn get_cars_handler(db_pool: web::Data<DbPool>) -> HttpResponse {
    let repo = CarRepositoryImpl::new(db_pool.get_ref().clone());
    match repo.find_all().await {
        Ok(cars) => HttpResponse::Ok().json(cars),
        Err(e) => {
            eprintln!("Error fetching cars: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch cars"
            }))
        }
    }
}
// GET /api/cars/vin/{vin} - получить автомобиль по VIN
pub async fn get_car_by_vin_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> HttpResponse {
    let repo = CarRepositoryImpl::new(db_pool.get_ref().clone());
    let vin = path.into_inner();

    match repo.find_by_vin(&vin).await {
        Ok(Some(car)) => HttpResponse::Ok().json(car),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Car not found"
        })),
        Err(e) => {
            eprintln!("Error fetching car by VIN {}: {}", vin, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch car"
            }))
        }
    }
}

// GET /api/cars/{id} - получить автомобиль по ID
pub async fn get_car_by_id_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<Uuid>
) -> HttpResponse {
    let repo = CarRepositoryImpl::new(db_pool.get_ref().clone());
    let id = path.into_inner();

    match repo.find_by_id(id).await {
        Ok(Some(car)) => HttpResponse::Ok().json(car),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Car not found"
        })),
        Err(e) => {
            eprintln!("Error fetching car {}: {}", id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch car"
            }))
        }
    }
}

// GET /api/cars/status/{status} - получить автомобили по статусу
pub async fn get_cars_by_status_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<CarStatus>
) -> HttpResponse {
    let repo = CarRepositoryImpl::new(db_pool.get_ref().clone());
    let status = path.into_inner();

    match repo.find_by_status(status).await {
        Ok(cars) => HttpResponse::Ok().json(cars),
        Err(e) => {
            eprintln!("Error fetching cars by status: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch cars"
            }))
        }
    }
}

// POST /api/cars - создать автомобиль
pub async fn create_car_handler(
    db_pool: web::Data<DbPool>,
    create_request: web::Json<CreateCarRequest>,
) -> HttpResponse {
    let repo = CarRepositoryImpl::new(db_pool.get_ref().clone());

    if let Err(validation_errors) = create_request.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Validation failed",
            "details": validation_errors
        }));
    }

    match repo.save(&create_request).await {
        Ok(car) => HttpResponse::Created().json(car),
        Err(e) => {
            eprintln!("Error creating car: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to create car"
            }))
        }
    }
}

// PUT /api/cars/{id} - обновить автомобиль
pub async fn update_car_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
    update_request: web::Json<UpdateCarRequest>,
) -> HttpResponse {
    let repo = CarRepositoryImpl::new(db_pool.get_ref().clone());
    let id = path.into_inner();

    if let Err(validation_errors) = update_request.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Validation failed",
            "details": validation_errors
        }));
    }

    match repo.update(id, &update_request).await {
        Ok(Some(car)) => HttpResponse::Ok().json(car),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Car not found"
        })),
        Err(e) => {
            eprintln!("Error updating car {}: {}", id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to update car"
            }))
        }
    }
}

// DELETE /api/cars/{id} - удалить автомобиль
pub async fn delete_car_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let repo = CarRepositoryImpl::new(db_pool.get_ref().clone());
    let id = path.into_inner();

    match repo.delete(id).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Car not found"
        })),
        Err(e) => {
            eprintln!("Error deleting car {}: {}", id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to delete car"
            }))
        }
    }
}

// PATCH /api/cars/{id}/status - обновить статус автомобиля
pub async fn update_car_status_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
    status: web::Json<CarStatus>,
) -> HttpResponse {
    let repo = CarRepositoryImpl::new(db_pool.get_ref().clone());
    let id = path.into_inner();
    let new_status = status.into_inner();

    match repo.update_status(id, new_status).await {
        Ok(Some(car)) => HttpResponse::Ok().json(car),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Car not found"
        })),
        Err(e) => {
            eprintln!("Error updating car status {}: {}", id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to update car status"
            }))
        }
    }
}