use actix_web::{web, HttpResponse};
use uuid::Uuid;
use validator::Validate;

use crate::{
    database::DbPool,
    models::{RequestStatus, CreatePurchaseRequest},
    repositories::{
        purchase_repository::PurchaseRepositoryImpl,
        car_repository::CarRepositoryImpl,
        customer_repository::CustomerRepositoryImpl,
    },
};
use crate::repositories::{CarRepository, CustomerRepository, PurchaseRepository};

// GET /api/purchases - получить все заявки
pub async fn get_purchases_handler(db_pool: web::Data<DbPool>) -> HttpResponse {
    let repo = PurchaseRepositoryImpl::new(db_pool.get_ref().clone());
    match repo.find_all().await {
        Ok(requests) => HttpResponse::Ok().json(requests),
        Err(e) => {
            eprintln!("Error fetching purchase requests: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch purchase requests"
            }))
        }
    }
}

// GET /api/purchases/{id} - получить заявку по ID
pub async fn get_purchase_by_id_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let repo = PurchaseRepositoryImpl::new(db_pool.get_ref().clone());
    let id = path.into_inner();

    match repo.find_by_id(id).await {
        Ok(Some(request)) => HttpResponse::Ok().json(request),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Purchase request not found"
        })),
        Err(e) => {
            eprintln!("Error fetching purchase request {}: {}", id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch purchase request"
            }))
        }
    }
}

// GET /api/purchases/customer/{customer_id} - получить заявки клиента
pub async fn get_purchases_by_customer_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let repo = PurchaseRepositoryImpl::new(db_pool.get_ref().clone());
    let customer_id = path.into_inner();

    match repo.find_by_customer_id(customer_id).await {
        Ok(requests) => HttpResponse::Ok().json(requests),
        Err(e) => {
            eprintln!("Error fetching purchases for customer {}: {}", customer_id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch purchase requests"
            }))
        }
    }
}

// GET /api/purchases/car/{car_id} - получить заявки на автомобиль
pub async fn get_purchases_by_car_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let repo = PurchaseRepositoryImpl::new(db_pool.get_ref().clone());
    let car_id = path.into_inner();

    match repo.find_by_car_id(car_id).await {
        Ok(requests) => HttpResponse::Ok().json(requests),
        Err(e) => {
            eprintln!("Error fetching purchases for car {}: {}", car_id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch purchase requests"
            }))
        }
    }
}

// POST /api/purchases - создать заявку на покупку
pub async fn create_purchase_handler(
    db_pool: web::Data<DbPool>,
    create_request: web::Json<CreatePurchaseRequest>,
) -> HttpResponse {
    let purchase_repo = PurchaseRepositoryImpl::new(db_pool.get_ref().clone());
    let car_repo = CarRepositoryImpl::new(db_pool.get_ref().clone());
    let customer_repo = CustomerRepositoryImpl::new(db_pool.get_ref().clone());

    if let Err(validation_errors) = create_request.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Validation failed",
            "details": validation_errors
        }));
    }

    // Проверяем что автомобиль существует
    match car_repo.find_by_id(create_request.car_id).await {
        Ok(Some(_)) => {}, // Автомобиль существует
        Ok(None) => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Car not found"
            }));
        }
        Err(e) => {
            eprintln!("Error checking car: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to validate car"
            }));
        }
    }

    // Проверяем что клиент существует
    match customer_repo.find_by_id(create_request.customer_id).await {
        Ok(Some(_)) => {}, // Клиент существует
        Ok(None) => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Customer not found"
            }));
        }
        Err(e) => {
            eprintln!("Error checking customer: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to validate customer"
            }));
        }
    }

    // Проверяем что нет активной заявки от этого клиента на эту машину
    match purchase_repo.exists_by_car_and_customer(create_request.car_id, create_request.customer_id).await {
        Ok(true) => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Purchase request already exists for this car and customer"
            }));
        }
        Err(e) => {
            eprintln!("Error checking existing request: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to check existing requests"
            }));
        }
        _ => {}
    }

    match purchase_repo.save(&create_request).await {
        Ok(request) => HttpResponse::Created().json(request),
        Err(e) => {
            eprintln!("Error creating purchase request: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to create purchase request"
            }))
        }
    }
}

// PATCH /api/purchases/{id}/status - обновить статус заявки
pub async fn update_purchase_status_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
    status: web::Json<RequestStatus>,
) -> HttpResponse {
    let repo = PurchaseRepositoryImpl::new(db_pool.get_ref().clone());
    let id = path.into_inner();
    let new_status = status.into_inner();

    match repo.update_status(id, new_status).await {
        Ok(Some(request)) => HttpResponse::Ok().json(request),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Purchase request not found"
        })),
        Err(e) => {
            eprintln!("Error updating purchase status {}: {}", id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to update purchase status"
            }))
        }
    }
}

// DELETE /api/purchases/{id} - удалить заявку
pub async fn delete_purchase_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let repo = PurchaseRepositoryImpl::new(db_pool.get_ref().clone());
    let id = path.into_inner();

    match repo.delete(id).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Purchase request not found"
        })),
        Err(e) => {
            eprintln!("Error deleting purchase request {}: {}", id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to delete purchase request"
            }))
        }
    }
}