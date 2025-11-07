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
        get_car_by_vin_handler
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
    })
        .bind((config.server.host.as_str(), config.server.port))?
        .run()
        .await
}