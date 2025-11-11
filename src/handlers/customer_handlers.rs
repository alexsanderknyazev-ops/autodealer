use actix_web::{web, HttpResponse};
use uuid::Uuid;
use validator::Validate;

use crate::{
    database::DbPool,
    models::CreateCustomerRequest,
    repositories::customer_repository::CustomerRepositoryImpl,
};
use crate::repositories::CustomerRepository;

// GET /api/customers - получить всех клиентов
pub async fn get_customers_handler(db_pool: web::Data<DbPool>) -> HttpResponse {
    let repo = CustomerRepositoryImpl::new(db_pool.get_ref().clone());
    match repo.find_all().await {
        Ok(customers) => HttpResponse::Ok().json(customers),
        Err(e) => {
            eprintln!("Error fetching customers: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch customers"
            }))
        }
    }
}

// GET /api/customers/{id} - получить клиента по ID
pub async fn get_customer_by_id_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let repo = CustomerRepositoryImpl::new(db_pool.get_ref().clone());
    let id = path.into_inner();

    match repo.find_by_id(id).await {
        Ok(Some(customer)) => HttpResponse::Ok().json(customer),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Customer not found"
        })),
        Err(e) => {
            eprintln!("Error fetching customer {}: {}", id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch customer"
            }))
        }
    }
}

// POST /api/customers - создать клиента
pub async fn create_customer_handler(
    db_pool: web::Data<DbPool>,
    create_request: web::Json<CreateCustomerRequest>,
) -> HttpResponse {
    let repo = CustomerRepositoryImpl::new(db_pool.get_ref().clone());

    if let Err(validation_errors) = create_request.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Validation failed",
            "details": validation_errors
        }));
    }
    
    match repo.exists_by_email(&create_request.email).await {
        Ok(true) => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Email already exists"
            }));
        }
        Err(e) => {
            eprintln!("Error checking email: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to check email"
            }));
        }
        _ => {}
    }

    match repo.save(&create_request).await {
        Ok(customer) => HttpResponse::Created().json(customer),
        Err(e) => {
            eprintln!("Error creating customer: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to create customer"
            }))
        }
    }
}

// PUT /api/customers/{id} - обновить клиента
pub async fn update_customer_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
    update_request: web::Json<CreateCustomerRequest>,
) -> HttpResponse {
    let repo = CustomerRepositoryImpl::new(db_pool.get_ref().clone());
    let id = path.into_inner();

    if let Err(validation_errors) = update_request.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Validation failed",
            "details": validation_errors
        }));
    }

    match repo.update(id, &update_request).await {
        Ok(Some(customer)) => HttpResponse::Ok().json(customer),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Customer not found"
        })),
        Err(e) => {
            eprintln!("Error updating customer {}: {}", id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to update customer"
            }))
        }
    }
}

// DELETE /api/customers/{id} - удалить клиента
pub async fn delete_customer_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let repo = CustomerRepositoryImpl::new(db_pool.get_ref().clone());
    let id = path.into_inner();

    match repo.delete(id).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Customer not found"
        })),
        Err(e) => {
            eprintln!("Error deleting customer {}: {}", id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to delete customer"
            }))
        }
    }
}