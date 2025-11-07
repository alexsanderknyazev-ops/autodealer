pub mod car_repository;
pub mod customer_repository;
pub mod purchase_repository;
pub mod part_repository;
pub mod brand_repository; // ← ДОБАВЛЯЕМ
pub mod car_model_repository; // ← ДОБАВЛЯЕМ

pub use car_repository::{CarRepository, CarRepositoryImpl};
pub use customer_repository::{CustomerRepository, CustomerRepositoryImpl};
pub use purchase_repository::{PurchaseRepository, PurchaseRepositoryImpl};
pub use part_repository::{PartRepository, PartRepositoryImpl};
pub use brand_repository::{BrandRepository, BrandRepositoryImpl}; // ← ДОБАВЛЯЕМ
pub use car_model_repository::{CarModelRepository, CarModelRepositoryImpl};