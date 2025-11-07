pub mod car;
pub mod customer;
pub mod purchase;
pub mod part;
pub mod brand; // ← ДОБАВЛЯЕМ
pub mod car_model; // ← ДОБАВЛЯЕМ
pub mod enums;

pub use car::{Car, CreateCarRequest, UpdateCarRequest};
pub use customer::{Customer, CreateCustomerRequest};
pub use purchase::{PurchaseRequest, CreatePurchaseRequest};
pub use part::{Part, CreatePartRequest, UpdatePartRequest};
pub use brand::{Brand, CreateBrandRequest, UpdateBrandRequest}; // ← ДОБАВЛЯЕМ
pub use car_model::{CarModel, CreateCarModelRequest, UpdateCarModelRequest}; // ← ДОБАВЛЯЕМ
pub use enums::{FuelType, Transmission, CarStatus, RequestStatus};