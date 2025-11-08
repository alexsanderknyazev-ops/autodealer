pub mod car_repository;
pub mod customer_repository;
pub mod purchase_repository;
pub mod part_repository;
pub mod brand_repository; // ← ДОБАВЛЯЕМ
pub mod car_model_repository;
pub mod work_repository;
pub mod service_campaign_repository;
pub mod warehouse_repository;

pub use car_repository::{CarRepository, CarRepositoryImpl};
pub use customer_repository::{CustomerRepository, CustomerRepositoryImpl};
pub use purchase_repository::{PurchaseRepository, PurchaseRepositoryImpl};
pub use part_repository::{PartRepository, PartRepositoryImpl};
pub use brand_repository::{BrandRepository, BrandRepositoryImpl};
pub use car_model_repository::{CarModelRepository, CarModelRepositoryImpl};
pub use work_repository::{WorkRepository, WorkRepositoryImpl};