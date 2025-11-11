use actix_web::{web, HttpResponse};
use uuid::Uuid;
use validator::Validate;

use crate::{
    database::DbPool,
    models::warehouse::{
        CreateWarehouseItemRequest, UpdateWarehouseItemRequest,
        StockMovementRequest, StockMovementType
    },
    repositories::warehouse_repository::WarehouseRepositoryImpl,
};
use crate::repositories::warehouse_repository::WarehouseRepository;

// GET /api/warehouse - получить все складские позиции
pub async fn get_warehouse_items_handler(db_pool: web::Data<DbPool>) -> HttpResponse {
    let repo = WarehouseRepositoryImpl::new(db_pool.get_ref().clone());
    match repo.find_all().await {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(e) => {
            eprintln!("Error fetching warehouse items: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch warehouse items"
            }))
        }
    }
}

// GET /api/warehouse/low-stock - получить позиции с низким запасом
pub async fn get_low_stock_items_handler(db_pool: web::Data<DbPool>) -> HttpResponse {
    let repo = WarehouseRepositoryImpl::new(db_pool.get_ref().clone());
    match repo.find_all_with_low_stock().await {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(e) => {
            eprintln!("Error fetching low stock items: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch low stock items"
            }))
        }
    }
}

// GET /api/warehouse/{id} - получить складскую позицию по ID
pub async fn get_warehouse_item_by_id_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let repo = WarehouseRepositoryImpl::new(db_pool.get_ref().clone());
    let id = path.into_inner();

    match repo.find_by_id(id).await {
        Ok(Some(item)) => HttpResponse::Ok().json(item),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Warehouse item not found"
        })),
        Err(e) => {
            eprintln!("Error fetching warehouse item {}: {}", id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch warehouse item"
            }))
        }
    }
}

// GET /api/warehouse/part/{part_id} - получить складскую позицию по ID запчасти
pub async fn get_warehouse_item_by_part_id_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let repo = WarehouseRepositoryImpl::new(db_pool.get_ref().clone());
    let part_id = path.into_inner();

    match repo.find_by_part_id(part_id).await {
        Ok(Some(item)) => HttpResponse::Ok().json(item),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Warehouse item not found for this part"
        })),
        Err(e) => {
            eprintln!("Error fetching warehouse item by part_id {}: {}", part_id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch warehouse item"
            }))
        }
    }
}

// GET /api/warehouse/article/{article} - получить складскую позицию по артикулу
pub async fn get_warehouse_item_by_article_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> HttpResponse {
    let repo = WarehouseRepositoryImpl::new(db_pool.get_ref().clone());
    let article = path.into_inner();

    match repo.find_by_article(&article).await {
        Ok(Some(item)) => HttpResponse::Ok().json(item),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Warehouse item not found"
        })),
        Err(e) => {
            eprintln!("Error fetching warehouse item by article {}: {}", article, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch warehouse item"
            }))
        }
    }
}

// GET /api/warehouse/location/{location} - получить складские позиции по местоположению
pub async fn get_warehouse_items_by_location_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> HttpResponse {
    let repo = WarehouseRepositoryImpl::new(db_pool.get_ref().clone());
    let location = path.into_inner();

    match repo.find_by_location(&location).await {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(e) => {
            eprintln!("Error fetching warehouse items by location {}: {}", location, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch warehouse items"
            }))
        }
    }
}

// POST /api/warehouse - создать складскую позицию
pub async fn create_warehouse_item_handler(
    db_pool: web::Data<DbPool>,
    create_request: web::Json<CreateWarehouseItemRequest>,
) -> HttpResponse {
    let repo = WarehouseRepositoryImpl::new(db_pool.get_ref().clone());

    if let Err(validation_errors) = create_request.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Validation failed",
            "details": validation_errors
        }));
    }
    match repo.exists_by_part_id(create_request.part_id).await {
        Ok(true) => {
            return HttpResponse::Conflict().json(serde_json::json!({
                "error": "Warehouse item for this part already exists"
            }));
        }
        Err(e) => {
            eprintln!("Error checking existing warehouse item: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to check existing warehouse item"
            }));
        }
        _ => {}
    }

    match repo.save(&create_request).await {
        Ok(item) => HttpResponse::Created().json(item),
        Err(e) => {
            eprintln!("Error creating warehouse item: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to create warehouse item"
            }))
        }
    }
}

// PUT /api/warehouse/{id} - обновить складскую позицию
pub async fn update_warehouse_item_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
    update_request: web::Json<UpdateWarehouseItemRequest>,
) -> HttpResponse {
    let repo = WarehouseRepositoryImpl::new(db_pool.get_ref().clone());
    let id = path.into_inner();

    if let Err(validation_errors) = update_request.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Validation failed",
            "details": validation_errors
        }));
    }

    match repo.update(id, &update_request).await {
        Ok(Some(item)) => HttpResponse::Ok().json(item),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Warehouse item not found"
        })),
        Err(e) => {
            eprintln!("Error updating warehouse item {}: {}", id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to update warehouse item"
            }))
        }
    }
}

// DELETE /api/warehouse/{id} - удалить складскую позицию
pub async fn delete_warehouse_item_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let repo = WarehouseRepositoryImpl::new(db_pool.get_ref().clone());
    let id = path.into_inner();

    match repo.delete(id).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Warehouse item not found"
        })),
        Err(e) => {
            eprintln!("Error deleting warehouse item {}: {}", id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to delete warehouse item"
            }))
        }
    }
}

// PUT /api/warehouse/{part_id}/stock - обновить запас (приход/расход/корректировка)
pub async fn update_stock_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
    movement_request: web::Json<StockMovementRequest>,
) -> HttpResponse {
    let repo = WarehouseRepositoryImpl::new(db_pool.get_ref().clone());
    let part_id = path.into_inner();

    if let Err(validation_errors) = movement_request.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Validation failed",
            "details": validation_errors
        }));
    }

    match repo.update_stock(part_id, &movement_request).await {
        Ok(Some(item)) => HttpResponse::Ok().json(item),
        Ok(None) => {
            let error_msg = match movement_request.movement_type {
                StockMovementType::Outgoing => "Warehouse item not found or insufficient stock",
                _ => "Warehouse item not found"
            };
            HttpResponse::NotFound().json(serde_json::json!({
                "error": error_msg
            }))
        }
        Err(e) => {
            eprintln!("Error updating stock for part {}: {}", part_id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to update stock"
            }))
        }
    }
}

// GET /api/warehouse/total-value - получить общую стоимость запасов
pub async fn get_total_inventory_value_handler(db_pool: web::Data<DbPool>) -> HttpResponse {
    let repo = WarehouseRepositoryImpl::new(db_pool.get_ref().clone());
    match repo.get_total_value().await {
        Ok(total_value) => HttpResponse::Ok().json(serde_json::json!({
            "total_value": total_value
        })),
        Err(e) => {
            eprintln!("Error calculating total inventory value: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to calculate total inventory value"
            }))
        }
    }
}