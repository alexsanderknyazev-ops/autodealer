use actix_web::{web, HttpResponse};
use uuid::Uuid;
use validator::Validate;

use crate::{
    database::DbPool,
    models::{CreateWorkRequest, UpdateWorkRequest},
    repositories::work_repository::WorkRepositoryImpl,
};
use crate::repositories::WorkRepository;

// GET /api/works - получить все работы
pub async fn get_works_handler(db_pool: web::Data<DbPool>) -> HttpResponse {
    let repo = WorkRepositoryImpl::new(db_pool.get_ref().clone());
    match repo.find_all().await {
        Ok(works) => HttpResponse::Ok().json(works),
        Err(e) => {
            eprintln!("Error fetching works: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch works"
            }))
        }
    }
}

// GET /api/works/{id} - получить работу по ID
pub async fn get_work_by_id_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let repo = WorkRepositoryImpl::new(db_pool.get_ref().clone());
    let id = path.into_inner();

    match repo.find_by_id(id).await {
        Ok(Some(work)) => HttpResponse::Ok().json(work),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Work not found"
        })),
        Err(e) => {
            eprintln!("Error fetching work {}: {}", id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch work"
            }))
        }
    }
}

// GET /api/works/article/{article} - получить работу по артикулу
pub async fn get_work_by_article_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> HttpResponse {
    let repo = WorkRepositoryImpl::new(db_pool.get_ref().clone());
    let article = path.into_inner();

    match repo.find_by_article(&article).await {
        Ok(Some(work)) => HttpResponse::Ok().json(work),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Work not found"
        })),
        Err(e) => {
            eprintln!("Error fetching work by article {}: {}", article, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch work"
            }))
        }
    }
}

// GET /api/works/brand/{brand_id} - получить работы по бренду
pub async fn get_works_by_brand_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let repo = WorkRepositoryImpl::new(db_pool.get_ref().clone());
    let brand_id = path.into_inner();

    match repo.find_by_brand(brand_id).await {
        Ok(works) => HttpResponse::Ok().json(works),
        Err(e) => {
            eprintln!("Error fetching works by brand {}: {}", brand_id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch works"
            }))
        }
    }
}

// GET /api/works/car-model/{car_model_id} - получить работы по модели автомобиля
pub async fn get_works_by_car_model_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let repo = WorkRepositoryImpl::new(db_pool.get_ref().clone());
    let car_model_id = path.into_inner();

    match repo.find_by_car_model(car_model_id).await {
        Ok(works) => HttpResponse::Ok().json(works),
        Err(e) => {
            eprintln!("Error fetching works by car model {}: {}", car_model_id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch works"
            }))
        }
    }
}

// GET /api/works/name/{name} - получить работы по названию
pub async fn get_works_by_name_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> HttpResponse {
    let repo = WorkRepositoryImpl::new(db_pool.get_ref().clone());
    let name = path.into_inner();

    match repo.find_by_name(&name).await {
        Ok(works) => HttpResponse::Ok().json(works),
        Err(e) => {
            eprintln!("Error fetching works by name {}: {}", name, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch works"
            }))
        }
    }
}

// POST /api/works - создать работу
pub async fn create_work_handler(
    db_pool: web::Data<DbPool>,
    create_request: web::Json<CreateWorkRequest>,
) -> HttpResponse {
    let repo = WorkRepositoryImpl::new(db_pool.get_ref().clone());

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
        Ok(work) => HttpResponse::Created().json(work),
        Err(e) => {
            eprintln!("Error creating work: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to create work"
            }))
        }
    }
}

// PUT /api/works/{id} - обновить работу
pub async fn update_work_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
    update_request: web::Json<UpdateWorkRequest>,
) -> HttpResponse {
    let repo = WorkRepositoryImpl::new(db_pool.get_ref().clone());
    let id = path.into_inner();

    if let Err(validation_errors) = update_request.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Validation failed",
            "details": validation_errors
        }));
    }
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
        Ok(Some(work)) => HttpResponse::Ok().json(work),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Work not found"
        })),
        Err(e) => {
            eprintln!("Error updating work {}: {}", id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to update work"
            }))
        }
    }
}

// DELETE /api/works/{id} - удалить работу
pub async fn delete_work_handler(
    db_pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let repo = WorkRepositoryImpl::new(db_pool.get_ref().clone());
    let id = path.into_inner();

    match repo.delete(id).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Work not found"
        })),
        Err(e) => {
            eprintln!("Error deleting work {}: {}", id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to delete work"
            }))
        }
    }
}