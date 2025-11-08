mod models;
mod config;
mod database;
mod repositories;
mod handlers;

use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};
use config::Config;
use database::create_db_pool;

use handlers::{
    car_handlers::{
        get_cars_handler, get_car_by_id_handler, get_cars_by_status_handler,
        create_car_handler, update_car_handler, delete_car_handler, update_car_status_handler,
        get_car_by_vin_handler,
        add_completed_campaign_handler, remove_completed_campaign_handler,
        clear_completed_campaigns_handler, get_pending_campaigns_handler,
        get_cars_by_completed_campaign_handler
    },
    customer_handlers::{
        get_customers_handler, get_customer_by_id_handler,
        create_customer_handler, update_customer_handler, delete_customer_handler
    },
    purchase_handlers::{
        get_purchases_handler, get_purchase_by_id_handler,
        get_purchases_by_customer_handler, get_purchases_by_car_handler,
        create_purchase_handler, update_purchase_status_handler, delete_purchase_handler
    },
    part_handlers::{
        get_parts_handler, get_part_by_id_handler, get_part_by_article_handler,
        get_parts_by_brand_handler, get_parts_by_car_model_handler, get_parts_by_vin_handler,
        create_part_handler, update_part_handler, delete_part_handler
    },
    brand_handlers::{
        get_brands_handler, get_brand_by_id_handler, get_brand_by_name_handler,
        get_brands_by_country_handler, create_brand_handler, update_brand_handler,
        delete_brand_handler
    },
    car_model_handlers::{
        get_car_models_handler, get_car_model_by_id_handler, get_car_models_by_brand_handler,
        get_car_models_by_name_handler, create_car_model_handler, update_car_model_handler,
        delete_car_model_handler
    },
    work_handlers::{
        get_works_handler, get_work_by_id_handler, get_work_by_article_handler,
        get_works_by_brand_handler, get_works_by_car_model_handler, get_works_by_name_handler,
        create_work_handler, update_work_handler, delete_work_handler
    },
    service_campaign_handlers::{
        get_service_campaigns_handler, get_service_campaign_by_id_handler,
        get_service_campaign_by_article_handler, get_service_campaigns_by_brand_handler,
        get_service_campaigns_by_car_model_handler, get_service_campaigns_by_status_handler,
        get_service_campaigns_by_mandatory_handler, get_service_campaigns_by_completed_handler,
        get_service_campaigns_by_vin_handler, create_service_campaign_handler,
        update_service_campaign_handler, delete_service_campaign_handler,
        update_service_campaign_status_handler, mark_service_campaign_completed_handler,
        mark_service_campaign_pending_handler
    },
    warehouse_handler::{
        get_warehouse_items_handler, get_low_stock_items_handler, get_warehouse_item_by_id_handler,
        get_warehouse_item_by_part_id_handler, get_warehouse_item_by_article_handler,
        get_warehouse_items_by_location_handler, create_warehouse_item_handler,
        update_warehouse_item_handler, delete_warehouse_item_handler, update_stock_handler,
        get_total_inventory_value_handler
    }
};
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("AutoDealer API is working!")
}

#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "message": "AutoDealer API is running"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    println!("🔧 Loading configuration...");
    let config = Config::from_env().expect("Failed to load configuration");

    println!("🗄️ Connecting to database...");
    let db_pool = create_db_pool(&config.database.url).await
        .expect("Failed to connect to database");

    println!("✅ Database connected successfully!");
    println!("🚀 Starting AutoDealer API on http://{}:{}", config.server.host, config.server.port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            // Базовые routes
            .service(hello)
            .service(health_check)
            // Car API routes
            .service(
                web::scope("/api/cars")
                    .route("", web::get().to(get_cars_handler))
                    .route("", web::post().to(create_car_handler))
                    .route("/{id}", web::get().to(get_car_by_id_handler))
                    .route("/{id}", web::put().to(update_car_handler))
                    .route("/{id}", web::delete().to(delete_car_handler))
                    .route("/status/{status}", web::get().to(get_cars_by_status_handler))
                    .route("/{id}/status", web::patch().to(update_car_status_handler))
                    .route("/vin/{vin}", web::get().to(get_car_by_vin_handler))
                    // Новые маршруты для сервисных кампаний
                    .route("/{car_id}/completed-campaigns/{campaign_id}", web::patch().to(add_completed_campaign_handler))
                    .route("/{car_id}/completed-campaigns/{campaign_id}", web::delete().to(remove_completed_campaign_handler))
                    .route("/{car_id}/completed-campaigns", web::delete().to(clear_completed_campaigns_handler))
                    .route("/{car_id}/pending-campaigns", web::get().to(get_pending_campaigns_handler))
                    .route("/completed-campaign/{campaign_id}", web::get().to(get_cars_by_completed_campaign_handler))
            )
            // Customer API routes
            .service(
                web::scope("/api/customers")
                    .route("", web::get().to(get_customers_handler))
                    .route("", web::post().to(create_customer_handler))
                    .route("/{id}", web::get().to(get_customer_by_id_handler))
                    .route("/{id}", web::put().to(update_customer_handler))
                    .route("/{id}", web::delete().to(delete_customer_handler))
            )
            // Purchase API routes
            .service(
                web::scope("/api/purchases")
                    .route("", web::get().to(get_purchases_handler))
                    .route("", web::post().to(create_purchase_handler))
                    .route("/{id}", web::get().to(get_purchase_by_id_handler))
                    .route("/{id}", web::delete().to(delete_purchase_handler))
                    .route("/{id}/status", web::patch().to(update_purchase_status_handler))
                    .route("/customer/{customer_id}", web::get().to(get_purchases_by_customer_handler))
                    .route("/car/{car_id}", web::get().to(get_purchases_by_car_handler))
            )
            // Parts API routes
            .service(
                web::scope("/api/parts")
                    .route("", web::get().to(get_parts_handler))
                    .route("", web::post().to(create_part_handler))
                    .route("/{id}", web::get().to(get_part_by_id_handler))
                    .route("/{id}", web::put().to(update_part_handler))
                    .route("/{id}", web::delete().to(delete_part_handler))
                    .route("/article/{article}", web::get().to(get_part_by_article_handler))
                    .route("/brand/{brand_id}", web::get().to(get_parts_by_brand_handler))
                    .route("/car-model/{car_model_id}", web::get().to(get_parts_by_car_model_handler))
                    .route("/vin/{vin}", web::get().to(get_parts_by_vin_handler))
            )
            // Brands API routes
            .service(
                web::scope("/api/brands")
                    .route("", web::get().to(get_brands_handler))
                    .route("", web::post().to(create_brand_handler))
                    .route("/{id}", web::get().to(get_brand_by_id_handler))
                    .route("/{id}", web::put().to(update_brand_handler))
                    .route("/{id}", web::delete().to(delete_brand_handler))
                    .route("/name/{name}", web::get().to(get_brand_by_name_handler))
                    .route("/country/{country}", web::get().to(get_brands_by_country_handler))
            )
            // Car Models API routes
            .service(
                web::scope("/api/car-models")
                    .route("", web::get().to(get_car_models_handler))
                    .route("", web::post().to(create_car_model_handler))
                    .route("/{id}", web::get().to(get_car_model_by_id_handler))
                    .route("/{id}", web::put().to(update_car_model_handler))
                    .route("/{id}", web::delete().to(delete_car_model_handler))
                    .route("/brand/{brand_id}", web::get().to(get_car_models_by_brand_handler))
                    .route("/name/{name}", web::get().to(get_car_models_by_name_handler))
            )
            // Works API routes
            .service(
                web::scope("/api/works")
                    .route("", web::get().to(get_works_handler))
                    .route("", web::post().to(create_work_handler))
                    .route("/{id}", web::get().to(get_work_by_id_handler))
                    .route("/{id}", web::put().to(update_work_handler))
                    .route("/{id}", web::delete().to(delete_work_handler))
                    .route("/article/{article}", web::get().to(get_work_by_article_handler))
                    .route("/brand/{brand_id}", web::get().to(get_works_by_brand_handler))
                    .route("/car-model/{car_model_id}", web::get().to(get_works_by_car_model_handler))
                    .route("/name/{name}", web::get().to(get_works_by_name_handler))
            )
            // Service Campaigns API routes
            .service(
                web::scope("/api/service-campaigns")
                    .route("", web::get().to(get_service_campaigns_handler))
                    .route("", web::post().to(create_service_campaign_handler))
                    .route("/{id}", web::get().to(get_service_campaign_by_id_handler))
                    .route("/{id}", web::put().to(update_service_campaign_handler))
                    .route("/{id}", web::delete().to(delete_service_campaign_handler))
                    .route("/article/{article}", web::get().to(get_service_campaign_by_article_handler))
                    .route("/brand/{brand_id}", web::get().to(get_service_campaigns_by_brand_handler))
                    .route("/car-model/{car_model_id}", web::get().to(get_service_campaigns_by_car_model_handler))
                    .route("/status/{status}", web::get().to(get_service_campaigns_by_status_handler))
                    .route("/mandatory/{is_mandatory}", web::get().to(get_service_campaigns_by_mandatory_handler))
                    .route("/completed/{is_completed}", web::get().to(get_service_campaigns_by_completed_handler))
                    .route("/vin/{vin}", web::get().to(get_service_campaigns_by_vin_handler))
                    .route("/{id}/status", web::patch().to(update_service_campaign_status_handler))
                    .route("/{id}/complete", web::patch().to(mark_service_campaign_completed_handler))
                    .route("/{id}/pending", web::patch().to(mark_service_campaign_pending_handler))
            )
            // Warehouse API routes
            .service(
                web::scope("/api/warehouse")
                    .route("", web::get().to(get_warehouse_items_handler))
                    .route("", web::post().to(create_warehouse_item_handler))
                    .route("/low-stock", web::get().to(get_low_stock_items_handler))
                    .route("/total-value", web::get().to(get_total_inventory_value_handler))
                    .route("/{id}", web::get().to(get_warehouse_item_by_id_handler))
                    .route("/{id}", web::put().to(update_warehouse_item_handler))
                    .route("/{id}", web::delete().to(delete_warehouse_item_handler))
                    .route("/part/{part_id}", web::get().to(get_warehouse_item_by_part_id_handler))
                    .route("/article/{article}", web::get().to(get_warehouse_item_by_article_handler))
                    .route("/location/{location}", web::get().to(get_warehouse_items_by_location_handler))
                    .route("/{part_id}/stock", web::put().to(update_stock_handler))
            )
    })
        .bind((config.server.host.as_str(), config.server.port))?
        .run()
        .await
}